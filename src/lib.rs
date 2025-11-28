#![no_std]
#![no_main]
extern crate alloc;

mod audio;
mod create;
mod creature;
mod gfx;
mod menu;

use alloc::format;
use audio::sound_off;
use dos_x::adlib::detect_adlib;
use dos_x::djgpp::dos::delay;
use dos_x::djgpp::dpmi::{__dpmi_int, __dpmi_regs};
use dos_x::vga::Palette;

use core::panic::PanicInfo;
use dos_x::vga::vsync;
use dos_x::{djgpp::stdlib::exit, println};
use tinyrand::{RandRange, Seeded};

use crate::audio::{adlib_notes_off, load_player, music_off};
use crate::create::{main_game, MainGameOutcome};
use crate::creature::CreatureParams;
use crate::gfx::{
    fade_out, init_palette, BitmapFont, CreatureAssets, COLOR_HIGHLIGHT, COLOR_WHITE,
};
use crate::menu::MenuOutcome;

/// 16x16 floppy disk icon, raw 8-bit indexed data
/// (already assumes game palette for B&W)
static FLOPPY_DATA: &[u8] = include_bytes!("../resources/floppy_16px.data");

/// Holder for all assets in the game,
/// so that they are readily available.
pub struct Assets {
    pub creature_assets: CreatureAssets,
    pub small_font: BitmapFont,
    pub big_font: BitmapFont,
    pub adlib_player: audio::AdlibPlayer,
}

#[derive(Debug, Copy, Clone)]
enum GameState {
    MainMenu,
    InGame,
    PresentingCreature,
}

#[no_mangle]
fn dos_main() {
    // process inputs
    for arg in dos_x::argv() {
        unsafe {
            let arg = core::ffi::CStr::from_ptr(*arg);
            if arg.to_bytes() == b"nosound" {
                sound_off();
                music_off();
            }
        }
    }

    // think of a way to seed the RNG later
    let seed = 0x67c9_0c6e_934c_aa87;

    let rng = tinyrand::Xorshift::seed(seed);
    run(rng);
}

fn run(mut rng: impl RandRange<u16>) {
    println!("Create-a-Creature by E_net4 (2025, v0.1.0)");

    unsafe {
        delay(1_000);
    }

    // disable the mouse
    unsafe {
        let mut regs: __dpmi_regs = core::mem::zeroed();
        regs.h.ah = 2;
        __dpmi_int(0x33, &mut regs);
    }

    dos_x::vga::set_video_mode_13h();
    unsafe {
        // clear screen (background color)
        dos_x::vga::draw_rect(0, 0, 320, 200, 253);
    }

    // initialize random creature
    let mut creature = CreatureParams::new_random(&mut rng);

    // grab palette and apply it to VGA display
    let mut palette = Palette::new([0u8; 768]);
    init_palette(&mut palette, &creature);

    unsafe {
        vsync();

        // clear screen (background color)
        dos_x::vga::draw_rect(0, 0, 320, 200, 253);

        // draw floppy disk onto the screen
        // (suggesting that the game is loading)
        dos_x::vga::blit_rect(FLOPPY_DATA, (16, 16), (0, 0, 16, 16), (152, 92));
    }

    match detect_adlib() {
        0 => {
            println!("No Adlib sound card detected, music disabled");
            music_off();
        }
        _ => {
            println!("Adlib sound card detected");
        }
    };
    let adlib_player = load_player();

    // load creature assets
    let creature_assets = CreatureAssets::load();

    let big_font = gfx::BitmapFont::big();
    let small_font = gfx::BitmapFont::small();

    let assets = Assets {
        creature_assets,
        small_font,
        big_font,
        adlib_player,
    };

    let mut state = GameState::MainMenu;
    loop {
        match state {
            GameState::MainMenu => {
                let outcome = menu::menu(&assets, &creature);
                match outcome {
                    MenuOutcome::Enter => {
                        state = GameState::InGame;
                    }
                    MenuOutcome::Exit => {
                        break;
                    }
                }
            }
            GameState::InGame => {
                let outcome = main_game(&assets, &mut creature, &mut palette);
                match outcome {
                    MainGameOutcome::Exit => break,
                    MainGameOutcome::SaveCreature => {
                        state = GameState::PresentingCreature;
                    }
                }
            }
            GameState::PresentingCreature => {
                present_creature(&assets, &creature);
                // and return to main menu
                state = GameState::MainMenu;
            }
        }
    }

    fade_out(&mut palette);

    // silence all music channels
    adlib_notes_off();

    // set back to text mode
    unsafe {
        dos_x::vga::set_video_mode(0x02);
    }

    println!("Thank you for playing!");
}

fn present_creature(assets: &Assets, creature: &CreatureParams) {
    let Assets {
        adlib_player,
        creature_assets,
        big_font,
        small_font,
        ..
    } = assets;

    let mut can_proceed = 128;
    unsafe {
        // clear screen (background color)
        dos_x::vga::draw_rect(0, 0, 320, 200, 253);
    }

    small_font.draw_text(86, 20, "You have created", gfx::COLOR_BLACK);

    // print creature name
    print_name(creature, big_font);

    // draw creature in center of screen
    creature_assets.draw_creature(creature, (320 - 32) / 2, (200 - 32) / 2);

    let mut keystate_enter = false;

    loop {
        unsafe {
            vsync();
        }

        adlib_player.poll(18_000);

        if can_proceed > 0 {
            can_proceed -= 1;
            continue;
        }

        small_font.draw_text(60, 165, "Press ENTER to continue", gfx::COLOR_BLACK);

        // check for ENTER key
        let key = dos_x::key::get_keypress();
        if key == 0x1c {
            // key pressed
            keystate_enter = true;
        } else if key == 0x9c && keystate_enter {
            // key released
            break;
        }
    }
}

/// print the creature's name at the center of the screen
/// (with an exclamation point)
pub(crate) fn print_name(creature: &CreatureParams, big_font: &BitmapFont) {
    let text = format!("{creature}!");

    // centered
    let x = (320 - (text.len() as i32 * 17)) / 2;
    big_font.draw_text(x - 1, 49, &text, COLOR_WHITE);
    big_font.draw_text(x, 50, text, COLOR_HIGHLIGHT);
}

#[panic_handler]
fn handle_panic(info: &PanicInfo) -> ! {
    unsafe {
        // reset video mode
        dos_x::vga::set_video_mode(0x02);
        println!("Program aborted: {}", info);
        println!("This is likely a bug! Please reach out:");
        println!("    https://github.com/Enet4/dos-createac/issues/new");
        // exit using libc
        exit(-1);
        core::hint::unreachable_unchecked()
    }
}
