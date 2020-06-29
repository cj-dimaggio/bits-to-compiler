bits 16
org 0x7c00
prologue:
mov bp, ($$ + 510)
mov sp, ($$ + 510)
call main
call epilogue

print:
    push bp
    mov bp, sp

    mov ah,0x0e
    int 0x10

    mov sp, bp
    pop bp
    ret
    
print_string:
push bp
mov bp, sp
push ax
mov ax, 0
push ax
.label_0:
mov ax, [bp - 2]
mov bx, ax
mov ax, [bp - 4]
mov al, [ebx + eax]
mov bx, ax
mov ax, 0
cmp ax, bx
mov ax, 0
setnz al
cmp ax, 0
je .label_1
mov ax, [bp - 2]
mov bx, ax
mov ax, [bp - 4]
mov al, [ebx + eax]
call print
mov ax, [bp - 4]
mov bx, ax
mov ax, 1
add ax, bx
mov [bp - 4], ax
jmp .label_0
.label_1:
mov sp, bp
pop bp
ret
main:
push bp
mov bp, sp
mov ax, string_0
call print_string
mov sp, bp
pop bp
ret
epilogue:
cli
hlt
string_0: db "Hello, World!", 0
times 510 - ($-$$) db 0
dw 0xaa55
