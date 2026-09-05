.section .text
.globl main
main:
    movq $10, %rdi
    movq $5, %rax
    add %rdi, %rax
    retq