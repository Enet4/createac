use core::cell::Cell;

use dos_x::{
    adlib,
    djgpp::{
        dos::delay,
        pc::{inportb, outportb},
    },
};
use opbinary::vgm::OplCommand;

static mut NO_SOUND: bool = false;
static mut NO_MUSIC: bool = false;

static MUSIC_VGM: &[u8] = include_bytes!("../resources/createac.vgm");

// Hz
const PIT_FREQUENCY: u32 = 0x1234DD;

/// disable sound effects
pub fn sound_off() {
    unsafe {
        NO_SOUND = true;
    }
}

/// disable music
pub fn music_off() {
    unsafe {
        NO_MUSIC = true;
    }
}

/// Play a very short click sound
pub fn play_click_1() {
    play_click_impl(1800, 2);
}

/// Play a click sound
pub fn play_click_2() {
    play_click_impl(1500, 4);
}

#[inline]
fn play_click_impl(countdown: u16, duration_ms: u32) {
    if unsafe { NO_SOUND } {
        return;
    }

    // use PC speaker
    unsafe {
        pc_speaker_on();

        play_note(countdown);
        delay(duration_ms);

        // turn off
        pc_speaker_off();
    }
}

#[inline]
unsafe fn play_note(countdown: u16) {
    outportb(0x42, (countdown & 0xff) as u8);
    outportb(0x42, (countdown >> 8) as u8);
}

#[inline]
unsafe fn pc_speaker_on() {
    unsafe {
        let inb = inportb(0x61);
        outportb(0x61, inb | 3); // enable speaker
        outportb(0x43, 0xb6); // set PIT
    }
}

#[inline(always)]
unsafe fn pc_speaker_off() {
    unsafe {
        outportb(0x61, 0);
    }
}

#[inline]
pub fn adlib_notes_off() {
    unsafe {
        for reg in 0xB0..0xB8 {
            adlib::write_command(reg, 0);
        }
        for reg in 0xC0..0xC8 {
            adlib::write_command(reg, 0);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlaybackState {
    Playing,
    Stopped,
}

/// An async processor of Adlib music playback.
pub struct AdlibPlayer {
    vgm: Option<opbinary::vgm::OplVgm>,
    cmd_index: core::cell::Cell<usize>,
    timer: core::cell::Cell<u32>,
}

fn samples_to_us(samples: u32) -> u32 {
    // VGM standard sample rate is 44100 Hz
    (samples * 10_000) / 441
}

impl AdlibPlayer {
    pub fn load(vgm_data: &[u8]) -> Self {
        if unsafe { NO_MUSIC } {
            return AdlibPlayer {
                vgm: None,
                cmd_index: Cell::new(0),
                timer: Cell::new(0),
            };
        }

        let vgm =
            opbinary::vgm::Vgm::from_bytes(vgm_data).expect("Could not read embedded VGM data");

        AdlibPlayer {
            vgm: Some(vgm.into_opl_vgm()),
            cmd_index: Cell::new(0),
            timer: Cell::new(0),
        }
    }

    pub fn poll(&self, delta_microseconds: u32) -> PlaybackState {
        let Some(vgm) = &self.vgm else {
            return PlaybackState::Stopped;
        };

        let timer = self.timer.get().saturating_sub(delta_microseconds);
        self.timer.set(timer);
        let mut cmd_index = self.cmd_index.get();

        while self.timer.get() == 0 && cmd_index < vgm.opl_commands.len() {
            let cmd = &vgm.opl_commands[cmd_index];
            match cmd {
                OplCommand::Opl3 {
                    port: 0,
                    address,
                    data,
                } => unsafe {
                    adlib::write_command_l(*address, *data);
                },
                OplCommand::Opl3 {
                    port: 1,
                    address,
                    data,
                } => unsafe {
                    adlib::write_command_r(*address, *data);
                },
                OplCommand::Opl2 { address, data }
                | OplCommand::Opl3 {
                    port: _,
                    address,
                    data,
                } => unsafe {
                    adlib::write_command(*address, *data);
                },
                OplCommand::Wait { samples } => {
                    self.timer.set(samples_to_us(*samples as u32));
                }
                OplCommand::SmallWait { n } => {
                    self.timer.set(samples_to_us(*n as u32 + 1));
                }
                OplCommand::Wait735 => {
                    self.timer.set(samples_to_us(735));
                }
                OplCommand::Wait882 => {
                    self.timer.set(samples_to_us(882));
                }
            }
            cmd_index += 1;
            self.cmd_index.set(cmd_index);
        }
        let cmd_index = self.cmd_index.get();
        if cmd_index >= vgm.opl_commands.len() {
            // loop
            self.cmd_index.set(cmd_index - vgm.opl_commands.len());
        }
        PlaybackState::Playing
    }
}

/// Initialize the Adlib music player,
/// loading the game music if music is enabled.
///
/// If music is disable, the returned dummy player does nothing.
pub fn load_player() -> AdlibPlayer {
    // load OPL data of music
    let mut player = AdlibPlayer::load(MUSIC_VGM);

    if let Some(vgm) = &mut player.vgm {
        // add a small waiting time at the end to avoid abrupt cut-off in the loop
        vgm.opl_commands.push(OplCommand::Wait735);
        vgm.opl_commands.push(OplCommand::Wait735);
        vgm.opl_commands.push(OplCommand::SmallWait { n: 255 });
    }

    player
}
