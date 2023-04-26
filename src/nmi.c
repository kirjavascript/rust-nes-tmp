// "The techniques llvm-mos uses for interrupt handling are somewhat unusual" - https://llvm-mos.org/wiki/C_interrupts

void render();

void wait_vblank(void) {
    asm(
        "lda #$00\n"
        "sta $80\n"
        "nop\n"
"checkForNmi:\n"
        "lda $80\n"
        "beq checkForNmi\n"
    );
}

__attribute__ ((no_isr)) void nmi(void) {
    asm(
            "  pha\n"
            "  txa\n"
            "  pha\n"
            "  tya\n"
            "  pha\n"
            "  lda #$1\n"
            "  sta $80\n"
       );
    render();
    asm(
            "  pla\n"
            "  tay\n"
            "  pla\n"
            "  tax\n"
            "  pla\n"
            "  rti\n"
       );
}
