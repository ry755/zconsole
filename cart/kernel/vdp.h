// VDP functions

#pragma once

#include <stdint.h>

void copy_tiles(uint8_t tile_table[], uint16_t number_of_tiles);
void clear_tile_table();
void fill_tile_table(uint16_t tile);
void set_tile(uint8_t x, uint8_t y, uint16_t tile);
//uint8_t get_tile(uint8_t x, uint8_t y);
void set_font_offset(uint16_t offset);
void print(uint8_t x, uint8_t y, char string[]);
