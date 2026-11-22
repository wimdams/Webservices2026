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



#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
