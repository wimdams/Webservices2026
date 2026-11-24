use font8x8::{BASIC_FONTS, UnicodeFonts};

const SCREEN_WIDTH: usize = 480;
const SCREEN_HEIGHT: usize = 272;

const TEXT_X: usize = 248;
const TEXT_Y: usize = 8;
const TEXT_RIGHT: usize = SCREEN_WIDTH - 8;

const CHAR_WIDTH: usize = 8;
const LINE_HEIGHT: usize = 10;

pub fn draw_char(
    x: usize,
    y: usize,
    ch: char,
    color: u16,
    mut put_pixel: impl FnMut(usize, usize, u16),
) {
    let Some(glyph) = BASIC_FONTS.get(ch) else {
        return;
    };

    for (dy, row) in glyph.iter().enumerate() {
        for dx in 0..8 {
            if row & (1u8 << dx) != 0 {
                put_pixel(x + dx, y + dy, color);
            }
        }
    }
}

pub fn draw_text(text: &str, color: u16, mut put_pixel: impl FnMut(usize, usize, u16)) {
    let mut x = TEXT_X;
    let mut y = TEXT_Y;

    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\n' {
            x = TEXT_X;
            y += LINE_HEIGHT;
            continue;
        }

        if x == TEXT_X && ch == ' ' {
            continue;
        }

        if ch != ' ' && ch != '\t' {
            let word_len = 1 + chars.clone().take_while(|c| !c.is_whitespace()).count();

            let remaining = TEXT_RIGHT.saturating_sub(x);
            let word_width = word_len * CHAR_WIDTH;

            if word_width > remaining && x != TEXT_X {
                x = TEXT_X;
                y += LINE_HEIGHT;
            }
        }

        if x + CHAR_WIDTH > TEXT_RIGHT {
            x = TEXT_X;
            y += LINE_HEIGHT;
        }

        if y + CHAR_WIDTH > SCREEN_HEIGHT {
            break;
        }

        if ch != '\t' {
            draw_char(x, y, ch, color, &mut put_pixel);
        }

        x += CHAR_WIDTH;
    }
}
