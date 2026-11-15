.section .rodata
msg:
    .asciz "Hello, world!\n"
.section .text
.globl _start
_start:
    # write(1, msg, 14)
    movq $1, %rax          # syscall №1 = write
    movq $1, %rdi          # arg1: fd = stdout
    leaq msg(%rip), %rsi   # arg2: адрес строки
    movq $14, %rdx         # arg3: длина строки
    syscall

    # exit(0)
    movq $60, %rax         # syscall №60 = exit
    movq $0, %rdi          # arg1: exit code = 0
    syscall