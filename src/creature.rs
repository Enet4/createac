//! Creature creation logic

use tinyrand::RandRange;

pub const NUM_SHAPES: u8 = 6;
pub const NUM_COLORS: u8 = 8;
pub const NUM_MOUTHS: u8 = 6;
pub const NUM_EYES: u8 = 6;
pub const NUM_LEGS: u8 = 6;
pub const NUM_ARMS: u8 = 6;

#[derive(Debug, Copy, Clone)]
pub struct CreatureParams {
    /// parameter 1: shape
    pub param1: u8,
    /// parameter 2: color
    pub param2: u8,
    /// parameter 3: eyes
    pub param3: u8,
    /// parameter 4: mouth
    pub param4: u8,
    /// parameter 5: legs
    pub param5: u8,
    /// parameter 6: arms
    pub param6: u8,
}

impl CreatureParams {
    pub fn new_random(rng: &mut impl RandRange<u16>) -> Self {
        CreatureParams {
            param1: rng.next_range(0..NUM_SHAPES as u16) as u8,
            param2: rng.next_range(0..NUM_COLORS as u16) as u8,
            param3: rng.next_range(0..NUM_EYES as u16) as u8,
            param4: rng.next_range(0..NUM_MOUTHS as u16) as u8,
            param5: rng.next_range(0..NUM_LEGS as u16) as u8,
            param6: rng.next_range(0..NUM_ARMS as u16) as u8,
        }
    }

    /// maps param2 to the main RGB color (in 0..64 range)
    pub fn body_color(&self) -> [u8; 3] {
        match self.param2 {
            // white
            0 => [0x3c, 0x3c, 0x3c],
            // red
            1 => [0x3c, 0x14, 0x14],
            // yellow
            2 => [0x3c, 0x3c, 0x14],
            // green
            3 => [0x14, 0x3c, 0x14],
            // cyan
            4 => [0x14, 0x3c, 0x3c],
            // blue
            5 => [0x16, 0x16, 0x3c],
            // magenta
            6 => [0x3c, 0x14, 0x3c],
            // brown
            7 => [0x2c, 0x1a, 0x12],
            // fallback to grey
            _ => [0x1f, 0x1f, 0x1f],
        }
    }

    /// create the palette slice for the creature's body colors
    pub fn body_colors(&self) -> [u8; 6] {
        let base_color = self.body_color();
        [
            // base color
            base_color[0],
            base_color[1],
            base_color[2],
            // darker shade
            base_color[0] / 2,
            base_color[1] / 2,
            base_color[2] / 2,
        ]
    }
}

/// The Display impl prints the creature's generated name
impl core::fmt::Display for CreatureParams {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // portion defined by the creature's color
        let display2 = match self.param2 {
            0 => "a",
            1 => "e",
            2 => "ey",
            3 => "i",
            5 => "o",
            6 => "ar",
            7 => "ur",
            4 => "or",
            _ => "Unknown",
        };

        // part 1
        match self.param1 {
            0 => write!(f, "Fl{display2}")?,
            1 => write!(f, "D{display2}n")?,
            2 => write!(f, "Bl{display2}")?,
            3 => write!(f, "N{display2}n")?,
            4 => write!(f, "Sn{display2}")?,
            5 => write!(f, "Yl{display2}m")?,
            _ => write!(f, "{display2}")?,
        };

        // defined by creature's limbs
        let display5 = match (self.param5, self.param6) {
            (0, 0) => "n",
            (1, 0) => "t",
            (2, 0) => "rl",
            (3, 0) => "d",
            (4, 0) => "p",
            (0, 1) => "sh",
            (0, 2) => "ch",
            (0, 3) => "gr",
            _ => "",
        };
        f.write_str(display5)?;

        // defined by creature's eyes
        let display4 = match self.param3 {
            0 => "i",
            1 => "o",
            2 => "u",
            3 => "e",
            4 => "a",
            5 => "yo",
            6 => "ya",
            _ => "",
        };
        f.write_str(display4)?;

        // defined by creature's mouth
        let display3 = match self.param4 {
            0 => "n",
            1 => "ty",
            2 => "d",
            3 => "m",
            4 => "z",
            5 => "b",
            6 => "pl",
            7 => "x",
            _ => "",
        };
        f.write_str(display3)
    }
}
