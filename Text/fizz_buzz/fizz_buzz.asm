section .text
global main
extern printf
main:
    sub rsp, 8
    xor eax, eax
_loop:
    mov rbx, rax
    push rbx
    xor edx, edx
    mov ebx, 15
    div ebx
    test edx, edx
    jz _fizzbuzz

    pop rax
    mov rbx, rax
    push rbx
    xor edx, edx
    mov ebx, 3
    div ebx
    test edx, edx
    jz _fizz

    pop rax
    mov rbx, rax
    push rbx
    xor edx, edx
    mov ebx, 5
    div ebx
    test edx, edx
    jz _buzz

    pop rax
    mov rbx, rax
    push rbx
    jmp _no_match
_fizzbuzz:
    lea rdi, [rel msg1]
    xor eax, eax
    call printf
    jmp _condition
_fizz:
    lea rdi, [rel msg2]
    xor eax, eax
    call printf
    jmp _condition
_buzz:
    lea rdi, [rel msg3]
    xor eax, eax
    call printf
    jmp _condition
_no_match:
    mov esi, eax
    lea rdi, [rel format]
    xor eax, eax
    call printf
_condition:
    pop rax
    add rax, 1
    cmp rax, 101
    jl _loop
_end:
    xor eax, eax
    add rsp, 8
    ret

section .rodata
    msg1: db "FizzBuzz", 10, 0
    msg2: db "Fizz", 10, 0
    msg3: db "Buzz", 10, 0
    format: db "%d", 10, 0
