; crt0.s for zconsole

    .module crt0
    .globl _main

    .area _HEADER (ABS)
    .org 0x0000
    jp init

    .org 0x0008
    ei
    reti

    .org 0x0010
    ei
    reti

    .org 0x0018
    ei
    reti

    .org 0x0020
    ei
    reti

    .org 0x0028
    ei
    reti

    .org 0x0030
    ei
    reti

    .org 0x0038
    ei
    reti

    .org 0x0100
init:
    ; set stack pointer to top of memory (pre-decrements, so set to zero)
    ld sp, #0x0000

    ; copy cartridge data into memory
    call copy_cart

    ; disable the cart
    call disable_cart

    ; initialize global variables
    call gsinit

    ; call main()
    call _main
hang:
    halt
    jp hang

copy_cart:
    ld hl, #0x0000
    ld de, #0x0000
    ld bc, #0x8000
    ldir

    ret

disable_cart:
    ld a, #0x00
    out (#0x00), a

    ret

    .area _GSINIT
gsinit:
    ld bc, #l__INITIALIZER
    ld a, b
    or a, c
    jr z, gsinit_next
    ld de, #s__INITIALIZED
    ld hl, #s__INITIALIZER
    ldir
gsinit_next:

    .area _GSFINAL
    ret

    ; ordering of segments for the linker
    .area _HOME
    .area _CODE
    .area _INITIALIZER
    .area _GSINIT
    .area _GSFINAL

    .area _DATA
    .area _INITIALIZED
    .area _BSEG
    .area _BSS
    .area _HEAP

    .area _CODE
