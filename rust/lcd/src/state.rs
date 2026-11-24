pub const TEXT_CAPACITY: usize = 1024;

pub struct LcdState {
    pub background_color: u16,
    pub text_color: u16,
    pub text: [u8; TEXT_CAPACITY],
    pub text_len: usize,
}

impl LcdState {
    pub const fn new() -> Self {
        Self {
            background_color: 0xFFFF,
            text_color: 0x0000,
            text: [0; TEXT_CAPACITY],
            text_len: 0,
        }
    }

    pub fn set_text(&mut self, text: &str) -> Result<(), ()> {
        let bytes = text.as_bytes();

        if bytes.len() > TEXT_CAPACITY {
            return Err(());
        }

        self.text[..bytes.len()].copy_from_slice(bytes);
        self.text_len = bytes.len();

        Ok(())
    }

    pub fn text(&self) -> &str {
        // set_text only accepts valid UTF-8.
        core::str::from_utf8(&self.text[..self.text_len]).unwrap_or("")
    }
}
