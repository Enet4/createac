use alloc::vec::Vec;
use alloc::{format, vec};
use dos_x::vga::{self, read_video_buffer_rect, vsync, Palette};
use minipng::{BitDepth, ImageData};

use crate::creature::CreatureParams;

pub const COLOR_HIGHLIGHT: u8 = 252;
pub const COLOR_BACKGROUND: u8 = 253;
pub const COLOR_WHITE: u8 = 254;
pub const COLOR_BLACK: u8 = 1;

// embed images into the binary
static CREATURE_SHAPES: &[u8] = include_bytes!("../resources/creature-shapes.png");
static CREATURE_EYES: &[u8] = include_bytes!("../resources/creature-eyes.png");
static CREATURE_MOUTHS: &[u8] = include_bytes!("../resources/creature-mouths.png");
static CREATURE_LEGS: &[u8] = include_bytes!("../resources/creature-legs.png");
static CREATURE_ARMS: &[u8] = include_bytes!("../resources/creature-arms.png");
static BIGFONT_PNG: &[u8] = include_bytes!("../resources/bigfont.png");
static SMALLFONT_PNG: &[u8] = include_bytes!("../resources/font.png");

#[derive(Debug)]
pub struct CreatureAssets {
    pub shapes_image: ImageAsset,
    pub eyes_image: ImageAsset,
    pub mouths_image: ImageAsset,
    pub legs_image: ImageAsset,
    pub arms_image: ImageAsset,
}

/// owned image asset (always 8-bit indexed)
pub struct ImageAsset {
    pub width: u32,
    pub height: u32,
    pub pixel_data: Vec<u8>,
    pub bit_depth: BitDepth,
    pub palette: Vec<u8>,
}

impl CreatureAssets {
    /// Load all creature assets.
    pub fn load() -> CreatureAssets {
        // load creature shapes
        let shapes_image = Self::load_asset(CREATURE_SHAPES);

        // load creature eyes
        let eyes_image = Self::load_asset(CREATURE_EYES);

        // load creature mouths
        let mouths_image = Self::load_asset(CREATURE_MOUTHS);

        // load creature legs
        let legs_image = Self::load_asset(CREATURE_LEGS);

        // load creature arms
        let arms_image = Self::load_asset(CREATURE_ARMS);

        CreatureAssets {
            shapes_image,
            eyes_image,
            mouths_image,
            legs_image,
            arms_image,
        }
    }

    /// Render the creature into a buffer.
    pub fn render_creature(&self, params: &CreatureParams, buffer: &mut [u8; 32 * 32]) {
        // render creature legs first
        let legs_index = params.param5 as u32;
        let legs_x = legs_index * 32;
        for j in 9..32 {
            for i in 0..32 {
                let src_offset = (j * self.legs_image.width + legs_x + i) as usize;
                let leg_pixel = self.legs_image.pixel_data[src_offset];
                if leg_pixel != 0 {
                    let dst_offset = (j * 32 + i) as usize;
                    buffer[dst_offset] = leg_pixel;
                }
            }
        }

        // render creature shape to buffer
        let shape_index = params.param1 as u32;
        let shape_x = shape_index * 32;

        for row in 1..31 {
            for col in 1..31 {
                let src_offset = (row * self.shapes_image.width + shape_x + col) as usize;
                let shape_pixel = self.shapes_image.pixel_data[src_offset];
                if shape_pixel != 0 {
                    let dst_offset = (row * 32 + col) as usize;
                    buffer[dst_offset] = shape_pixel;
                }
            }
        }

        // render arms
        let arms_index = params.param6 as u32;
        let arms_x = arms_index * 32;
        for j in 2..32 {
            for i in 0..32 {
                let src_offset = (j * self.arms_image.width + arms_x + i) as usize;
                let arm_pixel = self.arms_image.pixel_data[src_offset];
                if arm_pixel != 0 {
                    let dst_offset = (j * 32 + i) as usize;
                    buffer[dst_offset] = arm_pixel;
                }
            }
        }

        // render mouth
        let mouth_index = params.param4 as u32;
        let mouth_x = mouth_index * 32;

        for j in 5..28 {
            for i in 2..30 {
                let src_offset = (j * self.mouths_image.width + mouth_x + i) as usize;
                let mouth_pixel = self.mouths_image.pixel_data[src_offset];
                if mouth_pixel != 0 {
                    let dst_offset = (j * 32 + i) as usize;
                    buffer[dst_offset] = mouth_pixel;
                }
            }
        }

        // draw eyes on top
        let eyes_index = params.param3 as u32;
        let eyes_x = eyes_index * 32;

        // we use a tiny trick here, since we do not expect eye pixels around the boundaries
        for j in 2..25 {
            for i in 3..29 {
                let src_offset = (j * self.eyes_image.width + eyes_x + i) as usize;
                let eye_pixel = self.eyes_image.pixel_data[src_offset];
                if eye_pixel != 0 {
                    let dst_offset = (j * 32 + i) as usize;
                    buffer[dst_offset] = eye_pixel;
                }
            }
        }
    }

    /// Draw the creature to the screen at the given pixel coordinates.
    pub fn draw_creature(&self, params: &CreatureParams, x: i32, y: i32) {
        let mut buffer = [0; 32 * 32];

        unsafe {
            read_video_buffer_rect(&mut buffer, (x, y), (32, 32));
        }

        self.render_creature(params, &mut buffer);

        unsafe {
            dos_x::vga::blit_rect(&buffer, (32, 32), (0, 0, 32, 32), (x, y));
        }
    }

    fn load_asset(png_data: &[u8]) -> ImageAsset {
        // (first load header to know how much space to reserve)
        let h = minipng::decode_png_header(png_data).expect("Failed to read png header");
        let bytes_needed = h.required_bytes();
        let mut img_buffer: Vec<u8> = Vec::with_capacity(bytes_needed);
        img_buffer.resize(bytes_needed, 0);
        match minipng::decode_png(png_data, &mut img_buffer[..]) {
            Ok(image) => {
                if image.color_type() != minipng::ColorType::Indexed {
                    panic!("Image must be indexed");
                }
                let palette = Self::palette_from_imagedata(&image);

                let width = image.width();
                let height = image.height();
                let bit_depth = image.bit_depth();

                ImageAsset {
                    width,
                    height,
                    bit_depth,
                    pixel_data: img_buffer,
                    palette,
                }
            }
            Err(e) => {
                panic!("Could not decode PNG file: {}", e);
            }
        }
    }

    fn palette_from_imagedata(image: &ImageData) -> Vec<u8> {
        let mut palette = Vec::new();
        let bitdepth = image.bit_depth();
        for i in 0..bitdepth as u8 {
            let [r, g, b, _a] = image.palette(i);
            palette.push(r);
            palette.push(g);
            palette.push(b);
        }
        palette
    }
}

impl core::fmt::Debug for ImageAsset {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ImageData")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("bit_depth", &self.bit_depth)
            .field("palette_length", &format!("[u8; {}", &self.palette.len()))
            .finish()
    }
}

pub fn fade_out(palette: &mut Palette) {
    for _ in 0..32 {
        unsafe {
            for p in palette.0.iter_mut().take(768) {
                *p = p.saturating_sub(2);
            }
            vsync();
            palette.set();
        }
    }
}

pub fn draw_arrow_right(x: u32, y: u32, color: u8) {
    unsafe {
        dos_x::vga::put_pixel(x, y, color);
        dos_x::vga::put_pixel(x + 1, y + 1, color);
        dos_x::vga::put_pixel(x + 2, y + 2, color);
        dos_x::vga::put_pixel(x + 3, y + 3, color);
        dos_x::vga::put_pixel(x + 2, y + 4, color);
        dos_x::vga::put_pixel(x + 1, y + 5, color);
        dos_x::vga::put_pixel(x, y + 6, color);
    }
}

pub fn draw_arrow_left(x: u32, y: u32, color: u8) {
    unsafe {
        dos_x::vga::put_pixel(x + 3, y, color);
        dos_x::vga::put_pixel(x + 2, y + 1, color);
        dos_x::vga::put_pixel(x + 1, y + 2, color);
        dos_x::vga::put_pixel(x, y + 3, color);
        dos_x::vga::put_pixel(x + 1, y + 4, color);
        dos_x::vga::put_pixel(x + 2, y + 5, color);
        dos_x::vga::put_pixel(x + 3, y + 6, color);
    }
}

pub struct BitmapFont {
    pub pixeldata: Vec<u8>,
    pub char_width: u8,
    pub char_height: u8,
}

impl BitmapFont {
    /// the horizontal spacing added between characters when drawing text
    const H_SPACING: u8 = 1;

    const CHARS_PER_ROW: u8 = 9;

    pub fn big() -> Self {
        // 9x8 grid of 16x16 characters
        let h =
            minipng::decode_png_header(BIGFONT_PNG).expect("Failed to load big font PNG header");
        let mut pixeldata = vec![0; h.required_bytes()];
        let _image = minipng::decode_png(BIGFONT_PNG, &mut pixeldata[..])
            .expect("Failed to load big font PNG");

        BitmapFont {
            pixeldata,
            char_width: 16,
            char_height: 16,
        }
    }

    pub fn small() -> Self {
        // 9x8 grid of 8x8 characters
        let h = minipng::decode_png_header(SMALLFONT_PNG)
            .expect("Failed to load small font PNG header");
        let mut pixeldata = vec![0; h.required_bytes()];
        let _image = minipng::decode_png(SMALLFONT_PNG, &mut pixeldata[..])
            .expect("Failed to load small font PNG");

        BitmapFont {
            pixeldata,
            char_width: 8,
            char_height: 8,
        }
    }

    /// map a single text character
    /// to its pixel coordinates in the font sheet
    fn map_char_to_position(&self, char: u8) -> (u32, u32) {
        let (i, j) = match char {
            b'A'..=b'I' => {
                // row 0
                ((char - b'A') as u32, 0)
            }
            b'J'..=b'R' => {
                // row 1
                ((char - b'J') as u32, 1)
            }
            b'S'..=b'Z' => {
                // row 2
                ((char - b'S') as u32, 2)
            }
            b' ' => {
                // space at the end of row 2
                (8, 2)
            }
            b'.' => (0, 3),
            b'!' => (1, 3),
            b'?' => (2, 3),
            b':' => (3, 3),
            b',' => (4, 3),
            b'\'' => (5, 3),
            b'-' => (6, 3),
            // outline and filled cursors,
            // we'll use some other characters for this
            b'+' => (7, 3), // outline cursor
            b'*' => (8, 3), // filled cursor

            // non-bold letters, use lowercase for these
            b'a'..=b'i' => {
                // row 4
                ((char - b'a') as u32, 4)
            }
            b'j'..=b'r' => {
                // row 5
                ((char - b'j') as u32, 5)
            }
            b's'..=b'z' => {
                // row 6
                ((char - b's') as u32, 6)
            }

            b'0' => (8, 6),
            b'1'..=b'9' => ((char - b'1') as u32, 7),

            _ => {
                // unknown character, map to space
                (2, 2)
            }
        };

        (i * self.char_width as u32, j * self.char_height as u32)
    }

    pub fn draw_text(&self, x: i32, y: i32, text: impl AsRef<str>, color: u8) {
        let img_width = self.char_width as u32 * Self::CHARS_PER_ROW as u32;
        let cw = self.char_width as u32;
        let ch = self.char_height as u32;
        unsafe {
            let text = text.as_ref();
            for (i, c) in text.bytes().enumerate() {
                let (char_x, char_y) = self.map_char_to_position(c);
                let mut char_buffer = [0u8; 16 * 16];
                let target = (
                    x + i as i32 * (self.char_width as i32 + Self::H_SPACING as i32),
                    y,
                );
                // fill buffer from current video buffer
                vga::read_video_buffer_rect(&mut char_buffer, target, (cw, ch));

                // copy character pixel data into temporary buffer
                // if non-zero
                for row in 0..ch {
                    let src_offset = ((char_y + row) * img_width + char_x) as usize;
                    let dst_offset = (row * cw) as usize;
                    for col in 0..cw {
                        let pixel = self.pixeldata[src_offset + col as usize];
                        if pixel == 0 {
                            continue;
                        }
                        char_buffer[dst_offset + col as usize] = color;
                    }
                }
                // blit character
                dos_x::vga::blit_rect(&char_buffer, (cw, ch), (0, 0, cw, ch), target);
            }
        }
    }
}

pub fn init_palette(palette: &mut Palette, creature: &CreatureParams) {
    // initialize with zeros
    palette.0.fill(0);

    // set up palette:
    // 0: reserved for transparency
    // (setting to magenta for testing purposes)
    palette.0[0] = 0x30;
    palette.0[1] = 0x3f;
    palette.0[2] = 0x00;
    // 1: black
    // 2..=3: creature-dependent
    // 4: white
    // 5: light grey
    // 6: dark grey
    // 7: grey
    // 8: red
    // 9: darker red
    // 10: brown

    // 252: highlight color
    // 253: background color
    // 254: white
    // 255: black

    set_creature_palette(palette, creature);

    // always white
    palette.0[12] = 0x3c;
    palette.0[13] = 0x3c;
    palette.0[14] = 0x3c;

    // light grey
    palette.0[15] = 0x32;
    palette.0[16] = 0x32;
    palette.0[17] = 0x32;

    // dark grey
    palette.0[18] = 0x0f;
    palette.0[19] = 0x0f;
    palette.0[20] = 0x0f;

    // grey
    palette.0[21] = 0x1f;
    palette.0[22] = 0x1f;
    palette.0[23] = 0x1f;

    // red
    palette.0[24] = 0x3c;
    palette.0[25] = 0x03;
    palette.0[26] = 0x03;

    // darker red
    palette.0[27] = 0x1f;
    palette.0[28] = 0x01;
    palette.0[29] = 0x01;

    // brown
    palette.0[30] = 0x1f;
    palette.0[31] = 0x0e;
    palette.0[32] = 0x00;

    // highlight color (orange)
    palette.0[252 * 3] = 63;
    palette.0[252 * 3 + 1] = 36;
    palette.0[252 * 3 + 2] = 0;

    // background color (baby blue)
    palette.0[253 * 3] = 26;
    palette.0[253 * 3 + 1] = 50;
    palette.0[253 * 3 + 2] = 63;

    // ensure that the second last color (#254) is always white.
    palette.0[762] = 63;
    palette.0[763] = 63;
    palette.0[764] = 63;
    // the last color (#255) is always black.
    palette.set();
}

pub fn set_creature_palette(palette: &mut Palette, creature: &CreatureParams) {
    const COLOR_SAMPLES: usize = 6;
    let body_colors: [u8; COLOR_SAMPLES] = creature.body_colors();
    palette.0[6..6 + COLOR_SAMPLES].copy_from_slice(&body_colors);
    palette.set();
}
