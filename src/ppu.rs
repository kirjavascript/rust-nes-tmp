pub const CTRL_VRAM_INC: u8 = 0b100;
pub const _CTRL_NMI: u8 = 0b10000000;

pub fn write_ctrl(value: u8) {
    let p = 0x2000 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, value);
    }
}

pub fn and_ctrl(value: u8) {
    let p = 0x2000 as *mut u8;
    unsafe {
        let next = core::ptr::read_volatile(p) & value;
        core::ptr::write_volatile(p, next);
    }
}

pub fn or_ctrl(value: u8) {
    let p = 0x2000 as *mut u8;
    unsafe {
        let next = core::ptr::read_volatile(p) | value;
        core::ptr::write_volatile(p, next);
    }
}

pub fn write_mask(value: u8) {
    let p = 0x2001 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, value);
    }
}

pub fn write_addr(value: u16) {
    let p = 0x2006 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, (value >> 8) as u8);
        core::ptr::write_volatile(p, value as u8);
    }
}

pub fn write_data(value: u8) {
    let p = 0x2007 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, value);
    }
}

pub fn scroll(x: u8, y: u8) {
    let p = 0x2005 as *mut u8;
    unsafe {
        core::ptr::write_volatile(p, x);
        core::ptr::write_volatile(p, y);
    }
}

pub fn enable_nmi() {
    write_ctrl(0x80);
    write_mask(0x1E);
}

pub fn disable_nmi() {
    write_mask(0);
    write_ctrl(0);
}

#[inline(never)]
pub fn clear_nametable() {
    write_addr(0x2000);
    for _ in 0..0x400 {
        write_data(0);
    }
}

#[inline(never)]
pub fn draw_text(text: &str) {
    for ch in text.chars() {
        write_data(ch as u8 - 32);
    }
}

#[inline(never)]
pub fn draw_ascii(off: u16, ascii: &str) {
    for (i, line) in ascii[1..].split("\n").enumerate() {
        write_addr(off + (0x20 * i as u16));
        draw_text(line);
    }
}

#[inline(never)]
pub fn draw_box(x: u8, y: u8, w: u8, h: u8) {
    const BOX_TILES: u8 = 0x73;
    let offset = 0x2000 + (x as u16 + (y as u16 * 0x20));
    // -
    write_addr(offset);
    write_data(BOX_TILES);
    for _ in 0..w-2 {
        write_data(BOX_TILES + 5);
    }
    write_data(BOX_TILES + 1);
    // |
    write_addr(offset + 0x20);
    or_ctrl(CTRL_VRAM_INC);
    for _ in 0..h-2 {
        write_data(BOX_TILES + 4);
    }
    write_data(BOX_TILES + 2);
    // |
    write_addr(offset + 0x20 + w as u16 - 1);
    for _ in 0..h-2 {
        write_data(BOX_TILES + 4);
    }
    write_data(BOX_TILES + 3);
    and_ctrl(!CTRL_VRAM_INC);
    // _
    write_addr(offset + ((h as u16 - 1) * 0x20) + 1);
    for _ in 0..w-2 {
        write_data(BOX_TILES + 5);
    }
}

pub const STR_OFFSET: u8 = 0x10;

pub fn write_bytes(offset: u16, pal: &[u8]) {
    write_addr(offset);

    pal.iter().for_each(|byte| {
        write_data(*byte);
    });
}

#[allow(dead_code)]
pub const PAL_BG_0: u16 = 0x3f00;
#[allow(dead_code)]
pub const PAL_BG_1: u16 = 0x3f04;
#[allow(dead_code)]
pub const PAL_BG_2: u16 = 0x3f08;
#[allow(dead_code)]
pub const PAL_BG_3: u16 = 0x3f0C;
#[allow(dead_code)]
pub const PAL_SPRITE_0: u16 = 0x3f10;
#[allow(dead_code)]
pub const PAL_SPRITE_1: u16 = 0x3f14;
#[allow(dead_code)]
pub const PAL_SPRITE_2: u16 = 0x3f18;
#[allow(dead_code)]
pub const PAL_SPRITE_3: u16 = 0x3f1C;
