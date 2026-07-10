ENTRY(_start)
SECTIONS {
    . = 0x10000; /* match your base_addr */
    .text : { *(.text) }
    .data : { *(.data) }
    .bss  : { *(.bss) }
}
