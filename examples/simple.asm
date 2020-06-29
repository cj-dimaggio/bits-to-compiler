bits 16 ; tell NASM this is 16 bit code
org 0x7c00 ; tell NASM to start outputting stuff at offset 0x7c00
start:
    mov bp, ($$ + 510)
    mov sp, ($$ + 510)
    push 0xF8F0


    cmp (bp), 0xF8F0
    je print_success
    jmp print_fail

halt:
    cli ; clear interrupt flag
    hlt ; halt execution

print_success:
    mov si,success ; point si register to hello label memory location
    jmp print

print_fail:
    mov si,fail ; point si register to hello label memory location
    jmp print

print:
    mov ah,0x0e ; 0x0e means 'Write Character in TTY mode'
.loop:
    lodsb
    or al,al ; is al == 0 ?
    jz halt  ; if (al == 0) jump to halt label
    int 0x10 ; runs BIOS interrupt 0x10 - Video Services
    jmp .loop

success: db "SUCCESS",0
fail: db "FAIL",0

times 510 - ($-$$) db 0 ; pad remaining 510 bytes with zeroes
dw 0xaa55 ; magic bootloader magic - marks this 512 byte sector bootable!
