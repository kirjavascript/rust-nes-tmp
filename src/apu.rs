// https://www.nesdev.org/wiki/APU_basics

const APU: *mut u8 = 0x4000 as *mut u8;
const PULSE1: *mut u8 = 0x4000 as *mut u8;
#[allow(dead_code)]
const PULSE2: *mut u8 = 0x4004 as *mut u8;

static mut SFX: Sfx = Sfx::None;
static mut SFX_OFF: usize = 0;

#[derive(PartialEq)]
pub enum Sfx {
    ChangeScreen,
    MenuBoop,
    Pause,
    Shift,
    Rotate,
    Lock,
    LevelUp,
    Burn,
    FourLineClear,
    Topout,
    None,
}


pub fn init() {
    unsafe {
        [
            0x30,0x08,0x00,0x00,
            0x30,0x08,0x00,0x00,
            0x80,0x00,0x00,0x00,
            0x30,0x00,0x00,0x00,
            0x00,0x00,0x00,0x00,
        ].iter().enumerate().for_each(|(i, byte)|{
            *APU.offset(i as _) = *byte;
        });
        *APU.offset(0x15) = 0xF;
        *APU.offset(0x17) = 0x40;
    }
}

pub fn play_sfx(type_: Sfx) {
    unsafe {
        SFX = type_;
        SFX_OFF = 0;
    }
}

unsafe fn sfx_frame(p: *mut u8, hi: u8, lo: u8, dcvol: u8) -> bool {
    p.offset(2).write_volatile(lo);
    p.offset(3).write_volatile(hi); // only lower 3 bits matter
    p.write_volatile(dcvol);
    true
}

unsafe fn sfx_end(p: *mut u8) -> bool {
    p.write_volatile(0);
    false
}

#[allow(dead_code)]
unsafe fn noise_frame(tp: u8, vol: u8) -> bool {
    APU.offset(0xC).write_volatile(vol);
    APU.offset(0xE).write_volatile(tp);
    true
}

#[allow(dead_code)]
unsafe fn noise_end() -> bool {
    APU.offset(0xC).write_volatile(0b110000);
    APU.offset(0xE).write_volatile(0);
    false
}

pub fn run_sfx() {
    unsafe {
        if SFX != Sfx::None {
            let cont = match SFX {
                Sfx::ChangeScreen | Sfx::Pause => {
                    match SFX_OFF {
                        ..=5 => { sfx_frame(PULSE1, 1, 0x7C, 0b10111111) },
                        ..=10 => { sfx_frame(PULSE1, 1, 0xc4, 0b10111111) },
                        ..=15 => { sfx_frame(PULSE1, 0, 0xbf, 0b10111111) },
                        _ => { PULSE1.offset(3).write_volatile(7); sfx_end(PULSE1) }
                    }
                },
                Sfx::MenuBoop => {
                    match SFX_OFF {
                        ..=2 => { sfx_frame(PULSE1, 0, 0x90, 0b10110111) },
                        _ => { sfx_end(PULSE1) }
                    }
                },
                Sfx::Shift => {
                    match SFX_OFF {
                        ..=2 => { sfx_frame(PULSE1, 1, 0x7C, 0b10110111) },
                        _ => { sfx_end(PULSE1) }
                    }
                },
                Sfx::Lock => {
                    match SFX_OFF {
                        ..=2 => { sfx_frame(PULSE1, 5, 0x9d, 0b10110110) },
                        ..=3 => { sfx_frame(PULSE1, 6, 0xad, 0b10110110) },
                        _ => { sfx_end(PULSE1) }
                    }
                },
                Sfx::Rotate => {
                    match SFX_OFF {
                        ..=1 => { sfx_frame(PULSE1, 1, 0x7c, 0b10110110) },
                        ..=3 => { sfx_frame(PULSE1, 2, 0x1A, 0b10110000) },
                        ..=5 => { sfx_frame(PULSE1, 1, 0x7c, 0b10110110) },
                        _ => { sfx_end(PULSE1) }
                    }
                },
                Sfx::Burn | Sfx::FourLineClear | Sfx::LevelUp => {
                    const NOTES: &[u8] = &[0xfb,0xc4,0x93,0x67,0x3f,0x1c];

                    if SFX_OFF / 4 >= NOTES.len() {
                        PULSE1.offset(3).write_volatile(7);
                        sfx_end(PULSE1)
                    } else {
                        sfx_frame(PULSE1, 1, NOTES[SFX_OFF/4], 0b10111111)
                    }
                },
                Sfx::Topout => {
                    match SFX_OFF {
                        ..=5 => { sfx_frame(PULSE1, 4, 0x34, 0b10111110) },
                        ..=15 => { sfx_frame(PULSE1, 4, 0xb8, 0b10111000) },
                        ..=20 => { sfx_frame(PULSE1, 5, 0x4c, 0b10111110) },
                        ..=25 => { sfx_frame(PULSE1, 5, 0xf3, 0b10110110) },
                        _ => { sfx_end(PULSE1) }
                    }
                },
                Sfx::None => unreachable!(),
            };

            if cont {
                SFX_OFF += 1;
            } else {
                SFX = Sfx::None
            }
        }
    }
}
