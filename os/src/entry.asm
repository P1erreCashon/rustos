

.section .text.start
.globl _start
_start:

    la sp,boot_stack_top
    call rust_main


.section .bss.stack
boot_stack:
.space 4096*5
.globl boot_stack_top
boot_stack_top:

