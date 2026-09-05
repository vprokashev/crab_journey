.section .rodata
msg:
    .asciz "Hello, world!"
.section .text
.globl main
main:
    leaq msg(%rip), %rdi
    callq puts@PLT