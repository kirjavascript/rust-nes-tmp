/* default load chain */
/* ld.lld: /usr/local/bin/../mos-platform/nes-cnrom/lib/crt0.o */
/* ld.lld: /usr/local/bin/../mos-platform/nes/lib/libcrt0.a */
/* ld.lld: /usr/local/bin/../mos-platform/common/lib/libcrt.a */
/* ld.lld: /usr/local/bin/../mos-platform/common/lib/libc.a */
/* ld.lld: /usr/local/bin/../mos-platform/nes-cnrom/lib/link.ld */
/* ld.lld: /usr/local/bin/../mos-platform/nes-cnrom/lib/common.ld */
/* ld.lld: /usr/local/bin/../mos-platform/nes/lib/nes.ld */
/* ld.lld: /usr/local/bin/../mos-platform/common/lib/imag-regs.ld */
/* ld.lld: /usr/local/bin/../mos-platform/common/lib/text-sections.ld */
/* ld.lld: /usr/local/bin/../mos-platform/common/lib/rodata-sections.ld */
/* ld.lld: /usr/local/bin/../mos-platform/common/lib/data-sections.ld */
/* ld.lld: /usr/local/bin/../mos-platform/common/lib/zp-data-sections.ld */
/* ld.lld: /usr/local/bin/../mos-platform/common/lib/zp-bss-sections.ld */
/* ld.lld: /usr/local/bin/../mos-platform/common/lib/zp-bss-symbols.ld */
/* ld.lld: /usr/local/bin/../mos-platform/common/lib/zp-noinit.ld */
/* ld.lld: /usr/local/bin/../mos-platform/common/lib/zp-noinit-sections.ld */
/* ld.lld: /usr/local/bin/../mos-platform/common/lib/bss-sections.ld */
/* ld.lld: /usr/local/bin/../mos-platform/common/lib/bss-symbols.ld */
/* ld.lld: /usr/local/bin/../mos-platform/common/lib/noinit-sections.ld */
/* ld.lld: /usr/local/bin/../mos-platform/nes/lib/prg-ram-fixed.ld */
/* ld.lld: /usr/local/bin/../mos-platform/nes/lib/prg-rom-fixed.ld */
/* ld.lld: /usr/local/bin/../mos-platform/nes/lib/dpcm.ld */
/* ld.lld: /usr/local/bin/../mos-platform/nes/lib/ines.ld */
/* ld.lld: /usr/local/bin/../mos-platform/nes/lib/ines-header.ld */
/* ld.lld: /usr/local/bin/../mos-platform/nes/lib/ines-roms.ld */
/* ld.lld: /usr/local/bin/../mos-platform/nes/lib/c-in-ram.ld */

__chr_rom_size = 16;
/* __prg_rom_size = 32; */

/* Provide imaginary (zero page) registers. */
__rc0 = 0x00;
INCLUDE imag-regs.ld
ASSERT(__rc31 == 0x1f, "Inconsistent zero page map.")

ASSERT(__prg_rom_size == 1 << LOG2CEIL(__prg_rom_size),
       "PRG-ROM size must be a power of 2.")
ASSERT(__chr_rom_size == 0 || __chr_rom_size == 1 << LOG2CEIL(__chr_rom_size),
       "CHR-ROM size must be a power of 2.")
ASSERT(__prg_ram_size == 0 ||
       __prg_ram_size == 1 << LOG2CEIL(__prg_ram_size),
       "Nonempty PRG-RAM must be a power of 2.")
ASSERT(__prg_nvram_size == 0 ||
       __prg_nvram_size == 1 << LOG2CEIL(__prg_nvram_size),
       "Nonempty PRG-NVRAM must be a power of 2.")
ASSERT(__chr_ram_size == 0 ||
       __chr_ram_size == 1 << LOG2CEIL(__chr_ram_size),
       "Nonempty CHR-RAM must be a power of 2.")
ASSERT(__chr_nvram_size == 0 ||
       __chr_nvram_size == 1 << LOG2CEIL(__chr_nvram_size),
       "Nonempty CHR-NVRAM must be a power of 2.")

MEMORY {
  zp : ORIGIN = __rc31 + 1, LENGTH = 0x100 - (__rc31 + 1)

  /* reserved for sprites */
  sprites : ORIGIN = 0x0200, LENGTH = 0x0100

  ram : ORIGIN = 0x0300, LENGTH = 0x0500

  chr_rom : ORIGIN = 0x01000000, LENGTH = __chr_rom_size * 1024
}

REGION_ALIAS("c_ram", ram)
__stack = 0x07ff;

SECTIONS {
  .text : {
       INCLUDE text-sections.ld
       *(.nmi_begin)
       *(SORT_BY_INIT_PRIORITY(.nmi.* .nmi))
       *(.nmi_end)
  } >c_rom
  .rodata : { INCLUDE rodata-sections.ld } >c_rom
  .data : { INCLUDE data-sections.ld } >c_ram AT>c_rom
  __data_load_start = LOADADDR(.data);
  __data_size = SIZEOF(.data);
  .zp.data : { INCLUDE zp-data-sections.ld } >zp AT>c_rom
  __zp_data_load_start = LOADADDR(.zp.data);
  __zp_data_size = SIZEOF(.zp.data);
  .zp.bss (NOLOAD) : { INCLUDE zp-bss-sections.ld } >zp
  INCLUDE zp-bss-symbols.ld
  INCLUDE zp-noinit.ld
  .bss (NOLOAD) : { INCLUDE bss-sections.ld } >c_ram
  INCLUDE bss-symbols.ld
  .noinit (NOLOAD) : { INCLUDE noinit-sections.ld } >c_ram

  .chr_rom : { KEEP(*(.chr_rom .chr_rom.*)) } >chr_rom
}


INCLUDE prg-ram-fixed.ld

ASSERT(__prg_rom_size == 16 || __prg_rom_size == 32,
       "Fixed PRG-ROM only supports 16 KiB or 32 KiB")

__prg_rom_lma    = __prg_rom_size == 32 ? 0x8000 : 0xC000;
__prg_rom_offset = 0;

MEMORY { prg_rom (rx) : ORIGIN = __prg_rom_lma, LENGTH = __prg_rom_size * 1024 }

REGION_ALIAS("c_rom", prg_rom)
REGION_ALIAS("c_rom_lma", prg_rom)
REGION_ALIAS("c_rom_vma", prg_rom)

SECTIONS {
  .prg_rom_fixed : { *(.prg_rom_fixed .prg_rom_fixed.*) } >prg_rom
}
INCLUDE dpcm.ld
SECTIONS {
  .vector 0xfffa : { SHORT(nmi) SHORT(_start) SHORT(irq) } >prg_rom
}

OUTPUT_FORMAT {
  INCLUDE ines-header.ld
  FULL(prg_rom)
  FULL(chr_rom)
}


MEMORY {
  chr_rom_0   : ORIGIN = 0x01000000, LENGTH = __chr_rom_size >= 8    ? 0x2000 : 0
  chr_rom_1   : ORIGIN = 0x01002000, LENGTH = __chr_rom_size >= 16   ? 0x2000 : 0
}

SECTIONS {
  .chr_rom_0   : { KEEP(*(.chr_rom_0   .chr_rom_0.*)) }   >chr_rom_0
  .chr_rom_1   : { KEEP(*(.chr_rom_1   .chr_rom_1.*)) }   >chr_rom_1
}
