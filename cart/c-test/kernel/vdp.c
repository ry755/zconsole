// VDP routines

#include <stdint.h>

uint16_t font_offset = 0x0000;

#define WIDTH 256
#define HEIGHT 256
#define TILE_WIDTH 8
#define TILE_HEIGHT 8
#define TILES_WIDE WIDTH/TILE_WIDTH
#define TILES_TALL HEIGHT/TILE_HEIGHT
#define TILE_SIZE (TILE_WIDTH * TILE_HEIGHT) / 2

#define BACKGROUND_TABLE 0x0000
#define TILE_TABLE 0x1000

#define VDP_ADDR_LOW_PORT 0x10
#define VDP_ADDR_HIGH_PORT 0x11
#define VDP_DATA_PORT 0x12

volatile __sfr __at VDP_ADDR_LOW_PORT vdp_addr_low_io;
volatile __sfr __at VDP_ADDR_HIGH_PORT vdp_addr_high_io;
volatile __sfr __at VDP_DATA_PORT vdp_data_io;

#define vdp_set_address(addr) \
    { vdp_addr_low_io = addr & 0x00FF; vdp_addr_high_io = addr >> 8; }

// copy tiles into the VDP's memory
void copy_tiles(uint8_t tile_table[], uint8_t number_of_tiles) {
    vdp_set_address(TILE_TABLE);
    for (uint16_t i = 0; i < number_of_tiles * TILE_SIZE; i++) {
        vdp_data_io = tile_table[i];
    }
}

// set all tile pointers to tile 0
void clear_tile_table() {
    vdp_set_address(BACKGROUND_TABLE);
    for (uint16_t i = 0; i < TILES_TALL*TILES_WIDE; i++) {
        vdp_data_io = TILE_TABLE & 0xFF;
        vdp_data_io = TILE_TABLE >> 8;
    }
}

// set a tile at the specified coordinates
void set_tile(uint8_t x, uint8_t y, uint8_t tile) {
    uint16_t address = BACKGROUND_TABLE + ((y * TILES_WIDE) + x) * 2;
    uint16_t source_tile_address = TILE_TABLE + ((uint16_t)tile * TILE_SIZE);
    vdp_set_address(address);
    vdp_data_io = source_tile_address & 0xFF;
    vdp_data_io = source_tile_address >> 8;
}

// get a tile at the specified coordinates
// TODO: new system
//uint8_t get_tile(uint8_t x, uint8_t y) {
//    uint16_t address = BACKGROUND_TABLE + (y * TILES_WIDE) + x;
//    vdp_set_address(address);
//    return vdp_data_io;
//}

// set font offset in the tile table
void set_font_offset(uint16_t offset) {
    font_offset = offset;
}

void print(uint8_t x, uint8_t y, char string[]) {
    uint16_t address = BACKGROUND_TABLE + (y * TILES_WIDE) + x;
    vdp_set_address(address);
    while (*string) {
        vdp_data_io = *string + font_offset;
        string++;
    }
}
