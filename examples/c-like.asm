bits 16
org 0x7c00
prologue:
mov bp, ($$ + 510)
mov sp, ($$ + 510)
call main
main:
ret
epilogue:
hlt
times 510 - ($-$$) db 0
dw 0xaa55
