// need to import at least one C function to force the linker to work (?)
extern "C" {
    fn wait_vblank();
}

pub fn wait_for_vblank() {
    unsafe { wait_vblank() };
}

pub fn _set_chr_bank(bank: u8) {
    unsafe { *(0x8000 as *mut u8) = bank; }
}

pub const Right: u8 = 0x01;
pub const Left: u8 = 0x02;
pub const Down: u8 = 0x04;
pub const Up: u8 = 0x08;
pub const Start: u8 = 0x10;
pub const Select: u8 = 0x20;
pub const B: u8 = 0x40;
pub const A: u8 = 0x80;

static JOYPAD1: u16 = 0x4016;
static mut BUTTONS: u8 = 0;

pub fn poll_controller() {
    let joy = JOYPAD1 as *mut u8;

    // TODO: https://www.nesdev.org/wiki/Controller_reading_code#DPCM_Safety_using_Repeated_Reads

    unsafe {
        core::ptr::write_volatile(joy, 1);
        core::ptr::write_volatile(joy, 0);

        for _ in 0..8 {
            let a = core::ptr::read_volatile(joy);
            BUTTONS <<= 1;
            BUTTONS |= a & 1;
        }
    }
}

pub fn controller_buttons() -> u8 {
    unsafe { BUTTONS }
}
