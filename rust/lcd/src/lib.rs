#![no_std]

// mod color;
mod framebuffer;
// mod image;
// mod text;

use framebuffer::Framebuffer;

const LCD_WIDTH: u16 = 480;
const LCD_HEIGHT: u16 = 272;

const LCD_FB0_ADDR: u32 = 0xC0000000;
const LCD_FB1_ADDR: u32 = 0xC0040000;

const LTDC_BASE: usize = 0x4001_6800;
const LTDC_SRCR: *mut u32 = (LTDC_BASE + 0x24) as *mut u32;

const LTDC_LAYER0_CR: *mut u32 = (LTDC_BASE + 0x84) as *mut u32;
const LTDC_LAYER1_CR: *mut u32 = (LTDC_BASE + 0x104) as *mut u32;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn LCD_init() -> u8 {
    unsafe {
        LCD_clear(LCD_FB0_ADDR as *mut u16, LCD_WIDTH, LCD_HEIGHT, 0xffff);
        LCD_clear(LCD_FB1_ADDR as *mut u16, LCD_WIDTH, LCD_HEIGHT, 0xffff);
        LCD_layer_enable(0, true);
        LCD_layer_enable(1, false);
    }
    let mut fb0 = unsafe {
        Framebuffer::new(
            LCD_FB0_ADDR as *mut u16,
            LCD_WIDTH as usize,
            LCD_HEIGHT as usize,
        )
    };
    fb0.draw_line(
        (LCD_WIDTH / 2) as usize,
        0_usize,
        (LCD_WIDTH / 2) as usize,
        LCD_HEIGHT as usize,
        2,
        LCD_WIDTH as usize,
        LCD_HEIGHT as usize,
        0x0000,
    );

    let mut fb1 = unsafe {
        Framebuffer::new(
            LCD_FB0_ADDR as *mut u16,
            LCD_WIDTH as usize,
            LCD_HEIGHT as usize,
        )
    };
    fb1.draw_line(
        (LCD_WIDTH / 2) as usize,
        0_usize,
        (LCD_WIDTH / 2) as usize,
        LCD_HEIGHT as usize,
        2,
        LCD_WIDTH as usize,
        LCD_HEIGHT as usize,
        0x0000,
    );

    0
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

        LTDC_SRCR.write_volatile(1 << 1);
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
