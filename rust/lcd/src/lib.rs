#![no_std]

use core::{
    cell::RefCell,
    ffi::{CStr, c_char},
};

use cortex_m::interrupt::{self, Mutex};

mod framebuffer;
mod state;
mod text;

use framebuffer::Framebuffer;
use state::LcdState;

const LCD_WIDTH: usize = 480;
const LCD_HEIGHT: usize = 272;

const LCD_FB0_ADDR: usize = 0xC000_0000;
const LCD_FB1_ADDR: usize = 0xC004_0000;

const LTDC_BASE: usize = 0x4001_6800;
const LTDC_SRCR: *mut u32 = (LTDC_BASE + 0x24) as *mut u32;

const LTDC_LAYER0_CR: *mut u32 = (LTDC_BASE + 0x84) as *mut u32;
const LTDC_LAYER1_CR: *mut u32 = (LTDC_BASE + 0x104) as *mut u32;

static LCD_STATE: Mutex<RefCell<LcdState>> = Mutex::new(RefCell::new(LcdState::new()));

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

fn redraw(fb: &mut Framebuffer, state: &LcdState) {
    fb.clear(state.background_color);

    // TODO: draw_image(fb, &state.image);

    fb.fill_rect(LCD_WIDTH / 2 - 1, 0, 2, LCD_HEIGHT, 0x0000);

    text::draw_text(state.text(), state.text_color, |x, y, color| {
        fb.set_pixel(x, y, color)
    });
}

fn redraw_both(state: &LcdState) {
    for addr in [LCD_FB0_ADDR, LCD_FB1_ADDR] {
        let mut fb = unsafe { Framebuffer::new(addr as *mut u16, LCD_WIDTH, LCD_HEIGHT) };

        redraw(&mut fb, state);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn LCD_init() -> u8 {
    with_state(|state| {
        *state = LcdState::new();
        redraw_both(state);
    });

    LCD_layer_enable(0, true);
    LCD_layer_enable(1, false);

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn LCD_bg_set_color(color: Color) -> u8 {
    with_state(|state| {
        state.background_color = color.to_rgb565();
        redraw_both(state);
    });

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn LCD_txt_set_color(color: Color) -> u8 {
    with_state(|state| {
        state.text_color = color.to_rgb565();
        redraw_both(state);
    });

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

    with_state(|state| {
        if state.set_text(text).is_err() {
            return 1;
        }

        redraw_both(state);
        0
    })
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
pub extern "C" fn LCD_img_display_raw(_data: *const u16, _width: u16, _height: u16) -> u8 {
    1 // TODO
}

#[unsafe(no_mangle)]
pub extern "C" fn LCD_smiile_display(_filename: *const c_char) -> u8 {
    1 // TODO
}

#[unsafe(no_mangle)]
pub extern "C" fn LCD_bliss_display(_filename: *const c_char) -> u8 {
    1 // TODO
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
