; MiniCompiler Runtime Library (x86-64 NASM)
bits 64

section .text

; print_int(int value)
; System V ABI: value in RDI
global print_int
print_int:
    push rbp
    mov rbp, rsp
    sub rsp, 32          ; Buffer for string conversion

    mov rax, rdi
    mov rcx, 10
    lea rsi, [rbp-1]     ; Start from the end of the buffer
    mov byte [rsi], 0    ; Null terminator
    
    test rax, rax
    jns .positive
    neg rax
    mov r8, 1            ; Negative flag
    jmp .convert
.positive:
    mov r8, 0

.convert:
    xor rdx, rdx
    div rcx
    add dl, '0'
    dec rsi
    mov [rsi], dl
    test rax, rax
    jnz .convert

    test r8, r8
    jz .print
    dec rsi
    mov byte [rsi], '-'

.print:
    ; Count length
    mov rdx, rbp
    sub rdx, rsi
    dec rdx              ; length = rbp - rsi - 1

    ; write(1, rsi, rdx)
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; fd: stdout
    ; rsi is already set to the string
    ; rdx is already set to length
    syscall

    ; Print newline
    mov rax, 1
    mov rdi, 1
    lea rsi, [rel newline]
    mov rdx, 1
    syscall

    mov rsp, rbp
    pop rbp
    ret

; read_int() -> int
; Returns value in RAX
global read_int
read_int:
    ; Simple implementation: read from stdin and parse (placeholder)
    xor rax, rax
    ret

; exit(int status)
; status in RDI
global exit
exit:
    mov rax, 60         ; syscall: exit
    syscall

section .data
newline db 10
