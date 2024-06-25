// VDP functions

#pragma once

#include <stdint.h>

void copy_tiles(uint8_t tile_table[], uint8_t number_of_tiles);
void clear_tile_table();
void set_tile(uint8_t x, uint8_t y, uint8_t tile);
//uint8_t get_tile(uint8_t x, uint8_t y);
