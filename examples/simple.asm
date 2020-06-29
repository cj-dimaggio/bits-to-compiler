bits 16
org 0x7c00

prologue:
    mov bp, ($$ + 510)
    mov sp, ($$ + 510)
    call main
    call epilogue

main:
    mov ax, 1
    mov bx, ax
    mov ax, 1
    add ax, bx

    push 55
    push 22
    mov bx, sp ; https://board.flatassembler.net/topic.php?t=21150 can't access sp directly

    cmp ax, 2
    jz .ok
    .skip:
        add sp, 4
        ret

    .ok:
        call print_test
        add sp, 4
        ret

epilogue:
    call print_done
    cli
    hlt

print_done:
    mov si, done ; point si register to hello label memory location
    jmp print

print_test:
    mov si,test ; point si register to hello label memory location
    jmp print

print:
    mov ah,0x0e ; 0x0e means 'Write Character in TTY mode'
    .loop:
        lodsb
        or al,al ; is al == 0 ?
        jz .escape  ; if (al == 0) jump to halt label
        int 0x10 ; runs BIOS interrupt 0x10 - Video Services
        jmp .loop
    .escape:
        ret

done: db "DONE", 0
test: db "TEST", 0

times 510 - ($-$$) db 0
dw 0xaa55
