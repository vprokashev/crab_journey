# function fib(n) {
#   let a = 0;
#   let b = 1;
#
#   for (let i = 0; i < n; i++) {
#     const next = a + b;
#     a = b;
#     b = next;
#   }
#
#   return a;
# }


.section .text
.globl main
main:
    movq $10, %rdi # n
    movq $0, %rdx # a
    movq $1, %rbx # b
    movq $0, %rcx # index

loop:
    cmpq %rcx, %rdi
    jle exit
    movq %rdx, %rax # next = a
    addq %rbx, %rax # next = a + b
    movq %rbx, %rdx # a = b
    movq %rax, %rbx # b = next
    addq $1, %rcx
    jmp loop

exit:
    movq %rdx, %rax
    retq