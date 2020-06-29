bits 16
org 0x7c00

prologue:
    mov bp, ($$ + 510)
    mov sp, ($$ + 510)
    call main
    call epilogue

main:
    ; normalize stack
    push bp
    mov bp, sp

    ; let str = "Hello World";
    mov ax, string_1
    push ax

    ; let i = 0;
    mov ax, 0
    push ax

    .loop:
        ; str
        mov ax, [bp - 2]
        mov bx, ax

        ; i
        mov ax, [bp - 4]

        ; str[i]
        mov al, [ebx + eax] ; Scale not available with 16 bit registers

        mov bx, ax

        ; 0
        mov ax, 0

        ;!= 0
        cmp bx, ax
        mov ax, 0
        setnz al

        cmp ax, 0
        je .end

        ; print
        mov ax, bx
        call print
        
        ; i
        mov ax, [bp - 4]
        mov bx, ax

        ; 1
        mov ax, 1

        ; +
        add ax, bx

        mov [bp - 4], ax

        jmp .loop

    .end:
        mov sp, bp
        pop bp
        ret

print:
    ; normalize stack
    push bp
    mov bp, sp

    ; Argument expected to be passed in via ax reg
    mov ah,0x0e
    int 0x10

    mov sp, bp
    pop bp
    ret

epilogue:
    cli
    hlt

string_1: db "Hello, World!", 0

times 510 - ($-$$) db 0
dw 0xaa55

