OUTPUT_ARCH("riscv")
ENTRY(_start)
SECTIONS{

    . = 0x80200000;
    skernel = .;
    stext = .;
    .text : {
        *(.text.start)
        *(.text .text.*)
    }
    etext = .;

    . = ALIGN(4K);
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    erodata = .;

    . = ALIGN(4K);
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    edata = .;
    
    .bss : {
        . = ALIGN(4K);
        
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        
    }
    ebss = .;
    . = ALIGN(4K);
    ekernel = .;
    

}