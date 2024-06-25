# zconsole

**zconsole** is a Z80-powered fantasy console! This project is still a work-in-progress, so expect some rough edges.

The general specifications of the machine are:
- 64 KiB of general purpose RAM
- 32 KiB of cartridge space (copied into lower 32 KiB of RAM on startup by the crt0)
- 16 KiB of video RAM (mostly unused at the moment, but will provide plenty of space for sprites)
- 256x256 16-color display, addressable as 8x8-pixel tiles (full-screen bitmap graphics are possible!)
- 6-button controller input (D-Pad + Select + Back)

The specifications that are still undefined are the number and size of sprites (none at the moment), and the CPU clock speed (runs uncapped at the moment)
