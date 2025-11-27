use dos_x::{key, vga::vsync};

use crate::{
    audio::{play_click_1, play_click_2},
    creature::CreatureParams,
    gfx::{draw_arrow_left, draw_arrow_right, COLOR_BACKGROUND, COLOR_BLACK, COLOR_WHITE},
    Assets,
};

#[derive(Debug, Copy, Clone)]
pub enum MenuOutcome {
    /// Enter create-a-creature mode
    Enter,
    /// Exit the game
    Exit,
}

/// Show and operate the main menu
pub fn menu(assets: &Assets, creature: &CreatureParams) -> MenuOutcome {
    let Assets {
        adlib_player,
        creature_assets,
        big_font,
        small_font,
        ..
    } = assets;

    // simple menu screen with 2 choices
    let mut choice = 0;

    let mut keystate_up = false;
    let mut keystate_down = false;

    // clear background with background color
    unsafe {
        vsync();
        dos_x::vga::clear_screen(253);
    }
    big_font.draw_text(76, 23, "Create a", COLOR_WHITE);
    big_font.draw_text(77, 24, "Create a", COLOR_BLACK);
    crate::print_name(creature, big_font);

    creature_assets.draw_creature(creature, 144, 84);

    big_font.draw_text(102, 122, "Create!", COLOR_BLACK);
    big_font.draw_text(120, 148, "Exit", COLOR_BLACK);

    small_font.draw_text(136, 188, "Eduardo Pinho, 2025", COLOR_BLACK);

    loop {
        unsafe {
            vsync();
        }

        const ARROW_LEFT: u32 = 86;
        const ARROW_RIGHT: u32 = 234;
        // clear regions with selection arrow
        unsafe {
            dos_x::vga::draw_rect(ARROW_LEFT as i32, 125, 8, 32, COLOR_BACKGROUND);
            dos_x::vga::draw_rect(ARROW_RIGHT as i32, 125, 8, 32, COLOR_BACKGROUND);
        }

        let selection_y = 125 + choice as u32 * 25;
        draw_arrow_right(ARROW_LEFT, selection_y, COLOR_BLACK);
        draw_arrow_left(ARROW_RIGHT, selection_y, COLOR_BLACK);

        // check up down key presses
        let key = key::get_keypress();
        match key {
            k if (k & 0x80) != 0 => {
                keystate_up = false;
                keystate_down = false;
            }
            0x48 => {
                // up arrow
                if !keystate_up {
                    keystate_up = true;

                    // change choice
                    choice = 0;
                    play_click_1();
                }
            }
            0x50 => {
                // down
                if !keystate_down {
                    keystate_down = true;

                    // change choice
                    choice = 1;
                    play_click_1();
                }
            }
            0x1c => {
                play_click_2();
                return match choice {
                    0 => MenuOutcome::Enter,
                    1 => MenuOutcome::Exit,
                    _ => unreachable!(),
                };
            }
            0x01 => {
                // escape key
                play_click_2();
                return MenuOutcome::Exit;
            }
            _ => {
                // no-op
            }
        }

        adlib_player.poll(15_000);
    }
}
