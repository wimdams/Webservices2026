pub struct Framebuffer {
    ptr: *mut u16,
    width: usize,
    height: usize,
}

impl Framebuffer {
    /// # Safety
    /// ptr must refer to a writable framebuffer of at least
    /// width * height u16 pixels for the lifetime of this object.
    pub unsafe fn new(ptr: *mut u16, width: usize, height: usize) -> Self {
        Self { ptr, width, height }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u16) {
        if x >= self.width || y >= self.height {
            return;
        }

        unsafe {
            self.ptr.add(y * self.width + x).write_volatile(color);
        }
    }

    pub fn clear(&mut self, color: u16) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(x, y, color);
            }
        }
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u16) {
        let x_end = x.saturating_add(width).min(self.width);
        let y_end = y.saturating_add(height).min(self.height);

        for py in y..y_end {
            for px in x..x_end {
                self.set_pixel(px, py, color);
            }
        }
    }
}
