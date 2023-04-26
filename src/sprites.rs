use crate::ppu;

const ADDR: u16 = 0x200;
static mut INDEX: isize = 0;

pub fn dma() {
    let oamdma = 0x4014 as *mut u8;
    let oamaddr = 0x2003 as *mut u8;

    unsafe {
        core::ptr::write_volatile(oamaddr, 0);
        core::ptr::write_volatile(oamdma, (ADDR >> 8) as u8);
    }
}

pub fn clear() {
    let oam = ADDR as *mut u8;
    for i in 0..256 {
        unsafe { *oam.offset(i) = 0; }
    }
    unsafe { INDEX = 0; }
}

#[allow(dead_code)]
pub const PRIORITY: u8 = 0b100000;
#[allow(dead_code)]
pub const HFLIP: u8 = 0b1000000;
#[allow(dead_code)]
pub const VFLIP: u8 = 0b10000000;

pub fn add(x: u8, y: u8, tile: u8, attr: u8) {
    // attr is palette + flags
    let oam = ADDR as *mut u8;
    unsafe {
        *oam.offset(INDEX) = y;
        *oam.offset(INDEX + 1) = tile;
        *oam.offset(INDEX + 2) = attr;
        *oam.offset(INDEX + 3) = x;
        INDEX += 4;
    }
}
