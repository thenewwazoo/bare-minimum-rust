
MEMORY {
    BOOT (RX)  : ORIGIN = 0x00000000, LENGTH = 64K
    CODE (RX)  : ORIGIN = 0x08000000, LENGTH = 64K
    RAM (!RX) : ORIGIN = 0x1FFFEC00, LENGTH = 3K
}

SECTIONS {
    . = 0x00000000;
    .text : {
        KEEP(*(.vectors)) /* vectors must be placed first - page 63*/
        *(.startup*)
        *(.text*)
        *(.rodata*)
        . = ALIGN(4);
    } > BOOT

    .data : {
        . = ALIGN(4);
        _sdata = .;
        *(.fastrun*)
        *(.data*)
        . = ALIGN(4);
        _edata = .;
    } > RAM AT > CODE

    .bss : {
        . = ALIGN(4);
        _sbss = .;
        *(.bss*)
        *(COMMON)
        . = ALIGN(4);
        _ebss = .;
    } > RAM

    _estack = ORIGIN(RAM) + LENGTH(RAM); /* stack pointer start */
}
