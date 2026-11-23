#![no_std]

// mod color;
mod framebuffer;
// mod image;
// mod text;

use framebuffer::Framebuffer;

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

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
