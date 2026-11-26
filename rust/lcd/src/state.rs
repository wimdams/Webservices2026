pub const TEXT_CAPACITY: usize = 1024;

pub const DIRTY_MEDIA: u8 = 1 << 0;
pub const DIRTY_TEXT: u8 = 1 << 1;

pub const DIRTY_ALL: u8 = DIRTY_MEDIA | DIRTY_TEXT;

#[derive(Clone, Copy)]
pub struct LcdState {
    pub dirty: [u8; 2],
    pub front_buffer: usize,
    pub background_color: u16,
    pub text_color: u16,
    pub text: [u8; TEXT_CAPACITY],
    pub text_len: usize,
    pub image: Option<ImageState>,
    pub bliss: Option<BlissState>,
}

#[derive(Clone, Copy)]
pub struct ImageState {
    pub address: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Copy)]
pub struct BlissState {
    pub address: usize,
    pub len: usize,
    pub angle: f32,
}

impl LcdState {
    pub const fn new() -> Self {
        Self {
            dirty: [DIRTY_ALL; 2],
            front_buffer: 0,
            background_color: 0xFFFF,
            text_color: 0x0000,
            text: [0; TEXT_CAPACITY],
            text_len: 0,
            image: None,
            bliss: None,
        }
    }

    pub fn invalidate(&mut self, regions: u8) {
        self.dirty[0] |= regions;
        self.dirty[1] |= regions;
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
