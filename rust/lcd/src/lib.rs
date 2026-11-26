#![no_std]

use core::{
    cell::RefCell,
    ffi::{CStr, c_char},
};

use cortex_m::interrupt::{self, Mutex};

mod bliss;
mod dma2d;
mod framebuffer;
mod state;
mod text;

use framebuffer::Framebuffer;
use state::{BlissState, ImageState, LcdState};
use state::{DIRTY_ALL, DIRTY_MEDIA, DIRTY_TEXT};

const LCD_WIDTH: usize = 480;
const LCD_HEIGHT: usize = 272;

const LCD_FB0_ADDR: usize = 0xC0000000;
const LCD_FB1_ADDR: usize = 0xC0040000;

const LTDC_BASE: usize = 0x4001_6800;
const LTDC_SRCR: *mut u32 = (LTDC_BASE + 0x24) as *mut u32;

const LTDC_LAYER0_CR: *mut u32 = (LTDC_BASE + 0x84) as *mut u32;
const LTDC_LAYER1_CR: *mut u32 = (LTDC_BASE + 0x104) as *mut u32;

const LCD_BUFFERS: [usize; 2] = [LCD_FB0_ADDR, LCD_FB1_ADDR];

const LTDC_ISR: *const u32 = (LTDC_BASE + 0x38) as *const u32;

const LTDC_ICR: *mut u32 = (LTDC_BASE + 0x3C) as *mut u32;

const LTDC_IMR: u32 = 1 << 0;
const LTDC_VBR: u32 = 1 << 1;

const LTDC_LAYER0_CFBAR: *mut u32 = (LTDC_BASE + 0xAC) as *mut u32;

const LTDC_RRIF: u32 = 1 << 3;

static LCD_STATE: Mutex<RefCell<LcdState>> = Mutex::new(RefCell::new(LcdState::new()));

const MEDIA_WIDTH: usize = LCD_WIDTH / 2;
const DIVIDER_X: usize = MEDIA_WIDTH - 1;
const DIVIDER_WIDTH: usize = 2;
const TEXT_START_X: usize = DIVIDER_X + DIVIDER_WIDTH;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn to_rgb565(self) -> u16 {
        ((self.r as u16 >> 3) << 11) | ((self.g as u16 >> 2) << 5) | (self.b as u16 >> 3)
    }
}

fn with_state<R>(f: impl FnOnce(&mut LcdState) -> R) -> R {
    interrupt::free(|cs| {
        let mut state = LCD_STATE.borrow(cs).borrow_mut();
        f(&mut state)
    })
}

fn redraw_media(fb: &mut Framebuffer, state: &LcdState) {
    fb.fill_rect(0, 0, DIVIDER_X, LCD_HEIGHT, state.background_color);

    if let Some(image) = state.image {
        unsafe {
            fb.draw_image(image.address as *const u16, image.width, image.height);
        }
    } else if let Some(model_state) = state.bliss {
        let data = unsafe {
            core::slice::from_raw_parts(model_state.address as *const u8, model_state.len)
        };

        let model = unsafe { bliss::Bliss::from_validated(data) };

        bliss::render(fb, &model, model_state.angle, state.text_color);
    }

    fb.fill_rect(DIVIDER_X, 0, DIVIDER_WIDTH, LCD_HEIGHT, 0x0000);
}

fn redraw_text(fb: &mut Framebuffer, state: &LcdState) {
    fb.fill_rect(
        TEXT_START_X,
        0,
        LCD_WIDTH - TEXT_START_X,
        LCD_HEIGHT,
        state.background_color,
    );

    text::draw_text(state.text(), state.text_color, |x, y, color| {
        fb.set_pixel(x, y, color)
    });
}

fn redraw(fb: &mut Framebuffer, state: &LcdState) {
    redraw_media(fb, state);
    redraw_text(fb, state);
}

fn redraw_back() {
    let mut snapshot = with_state(|state| *state);

    let back = snapshot.front_buffer ^ 1;
    let dirty = snapshot.dirty[back];

    if dirty == 0 {
        return;
    }

    let mut fb = unsafe { Framebuffer::new(LCD_BUFFERS[back] as *mut u16, LCD_WIDTH, LCD_HEIGHT) };

    if dirty & DIRTY_MEDIA != 0 {
        redraw_media(&mut fb, &snapshot);
    }

    if dirty & DIRTY_TEXT != 0 {
        redraw_text(&mut fb, &snapshot);
    }

    swap_buffers(&mut snapshot);

    with_state(|state| {
        state.front_buffer = snapshot.front_buffer;
        state.dirty[back] &= !dirty;
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn LCD_init() -> u8 {
    unsafe {
        let cr = LTDC_LAYER0_CR.read_volatile();
        LTDC_LAYER0_CR.write_volatile(cr | 1);

        let cr = LTDC_LAYER1_CR.read_volatile();
        LTDC_LAYER1_CR.write_volatile(cr & !1);

        LTDC_LAYER0_CFBAR.write_volatile(LCD_FB0_ADDR as u32);

        LTDC_SRCR.write_volatile(1);
    }

    with_state(|state| {
        *state = LcdState::new();
        state.invalidate(DIRTY_ALL);
    });

    redraw_back();

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn LCD_bg_set_color(color: Color) -> u8 {
    with_state(|state| {
        state.background_color = color.to_rgb565();
        state.invalidate(DIRTY_ALL);
    });

    redraw_back();

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn LCD_txt_set_color(color: Color) -> u8 {
    with_state(|state| {
        state.text_color = color.to_rgb565();
        state.invalidate(DIRTY_ALL);
    });

    redraw_back();

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn LCD_txt_display(text: *const c_char) -> u8 {
    if text.is_null() {
        return 1;
    }

    let Ok(text) = (unsafe { CStr::from_ptr(text) }).to_str() else {
        return 1;
    };

    let result = with_state(|state| {
        if state.set_text(text).is_err() {
            return 1;
        }

        state.invalidate(DIRTY_TEXT);

        0
    });

    if result == 0 {
        redraw_back();
    }

    result
}

#[unsafe(no_mangle)]
pub extern "C" fn LCD_layer_enable(layer: u8, enable: bool) {
    let cr = match layer {
        0 => LTDC_LAYER0_CR,
        1 => LTDC_LAYER1_CR,
        _ => return,
    };

    unsafe {
        let value = cr.read_volatile();

        if enable {
            cr.write_volatile(value | 1);
        } else {
            cr.write_volatile(value & !1);
        }

        // Reload at vertical blanking.
        LTDC_SRCR.write_volatile(1 << 1);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn LCD_clear(buffer: *mut u16, width: u16, height: u16, color: u16) {
    if buffer.is_null() {
        return;
    }

    let mut fb = unsafe { Framebuffer::new(buffer, width as usize, height as usize) };

    fb.clear(color);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn LCD_fill_rect(
    buffer: *mut u16,
    width: u16,
    height: u16,
    x: u16,
    y: u16,
    rect_width: u16,
    rect_height: u16,
    color: u16,
) {
    if buffer.is_null() {
        return;
    }

    let mut fb = unsafe { Framebuffer::new(buffer, width as usize, height as usize) };

    fb.fill_rect(
        x as usize,
        y as usize,
        rect_width as usize,
        rect_height as usize,
        color,
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn LCD_img_display_raw(data: *const u16, width: u16, height: u16) -> u8 {
    let width = width as usize;
    let height = height as usize;

    if data.is_null() {
        with_state(|state| {
            state.image = None;
            state.bliss = None;
            state.invalidate(DIRTY_MEDIA);
        });

        redraw_back();

        return 0;
    }

    if width == 0 || height == 0 || width > LCD_WIDTH / 2 || height > LCD_HEIGHT {
        return 1;
    }

    with_state(|state| {
        state.image = Some(ImageState {
            address: data as usize,
            width,
            height,
        });

        state.bliss = None;
        state.invalidate(DIRTY_MEDIA);
    });

    redraw_back();

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn LCD_smiile_display(_filename: *const c_char) -> u8 {
    1 // TODO
}

#[unsafe(no_mangle)]
pub extern "C" fn LCD_bliss_display(_filename: *const c_char) -> u8 {
    1 // TODO
}

/// # Safety
/// data must point to len readable bytes that remain
/// valid and unchanged until the model is replaced.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LCD_bliss_display_raw(data: *const u8, len: u32) -> u8 {
    if data.is_null() || len == 0 {
        return 1;
    }

    let bytes = unsafe { core::slice::from_raw_parts(data, len as usize) };

    if bliss::Bliss::parse(bytes).is_err() {
        return 1;
    }

    with_state(|state| {
        state.image = None;

        state.bliss = Some(BlissState {
            address: data as usize,
            len: len as usize,
            angle: 0.0,
        });

        state.invalidate(DIRTY_MEDIA);
    });

    redraw_back();

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn LCD_bliss_tick(elapsed_ms: u32) {
    if elapsed_ms == 0 {
        return;
    }

    let changed = with_state(|state| {
        let Some(model) = state.bliss.as_mut() else {
            return false;
        };

        model.angle += elapsed_ms as f32 * 0.0004;

        if model.angle >= core::f32::consts::TAU {
            model.angle %= core::f32::consts::TAU;
        }

        state.invalidate(DIRTY_MEDIA);

        true
    });

    if changed {
        redraw_back();
    }
}

fn swap_buffers(state: &mut LcdState) {
    let back_buffer = state.front_buffer ^ 1;

    cortex_m::asm::dsb();

    unsafe {
        LTDC_LAYER0_CFBAR.write_volatile(LCD_BUFFERS[back_buffer] as u32);

        LTDC_SRCR.write_volatile(LTDC_VBR);

        let mut timeout = 10_000_000u32;

        while LTDC_SRCR.read_volatile() & LTDC_VBR != 0 {
            if timeout == 0 {
                debug_print(c"VBlank timeout\r\n".as_ptr());

                return;
            }

            timeout -= 1;
            core::hint::spin_loop();
        }
    }

    state.front_buffer = back_buffer;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
