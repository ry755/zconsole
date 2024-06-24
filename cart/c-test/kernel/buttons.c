// button routines

#include <stdint.h>

#define BUTTONS_PORT 0x20

volatile __sfr __at BUTTONS_PORT buttons_io;

uint8_t get_buttons() {
    return buttons_io;
}
