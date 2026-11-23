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

    pub fn draw_line(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        thickness: usize,
        width: usize,
        height: usize,
        color: u16,
    ) {
        if width == 0 || height == 0 {
            return;
        }

        let (mut x, mut y) = (x1 as isize, y1 as isize);
        let (x2, y2) = (x2 as isize, y2 as isize);

        let dx = (x2 - x).abs();
        let sx = if x < x2 { 1 } else { -1 };
        let dy = -(y2 - y).abs();
        let sy = if y < y2 { 1 } else { -1 };
        let mut error = dx + dy;

        let brush = thickness.max(1) as isize;
        let before = (brush - 1) / 2;
        let after = brush / 2;

        loop {
            for oy in -before..=after {
                for ox in -before..=after {
                    let px = x + ox;
                    let py = y + oy;

                    if px >= 0 && py >= 0 && px < width as isize && py < height as isize {
                        self.set_pixel(px as usize, py as usize, color);
                    }
                }
            }

            if x == x2 && y == y2 {
                break;
            }

            let twice_error = 2 * error;

            if twice_error >= dy {
                error += dy;
                x += sx;
            }
            if twice_error <= dx {
                error += dx;
                y += sy;
            }
        }
    }
}
