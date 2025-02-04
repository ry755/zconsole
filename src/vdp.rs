// vdp.rs

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 256;
const TILE_WIDTH: usize = 8;
const TILE_HEIGHT: usize = 8;
const TILES_WIDE: usize = WIDTH / TILE_WIDTH;
const TILES_TALL: usize = HEIGHT / TILE_HEIGHT;
const TILE_SIZE: usize = (TILE_WIDTH * TILE_HEIGHT) / 2;

const BACKGROUND_TABLE: usize = 0x0000;
//const TILE_TABLE: usize = 0x1000;

pub type VdpRam = [u8; 16 * 1024];

pub struct Vdp {
    ram: VdpRam,
    ram_ptr: u16,

    background: Vec<u8>,
}

impl Vdp {
    pub fn new() -> Self {
        Vdp {
            ram: [0; 16 * 1024],
            ram_ptr: 0,
            background: vec![0; (WIDTH * HEIGHT * 4) as usize],
        }
    }

    pub fn set_pointer_low(&mut self, byte: u8) {
        self.ram_ptr = self.ram_ptr & 0xFF00;
        self.ram_ptr = self.ram_ptr | byte as u16;
    }

    pub fn set_pointer_high(&mut self, byte: u8) {
        self.ram_ptr = self.ram_ptr & 0x00FF;
        self.ram_ptr = self.ram_ptr | (byte as u16) << 8;
    }

    pub fn read_memory(&mut self) -> u8 {
        let byte = self.ram[self.ram_ptr as usize];
        self.ram_ptr += 1;
        byte
    }

    pub fn write_memory(&mut self, byte: u8) {
        //println!("writing {:#X} to {:#X}", byte, self.ram_ptr);
        self.ram[self.ram_ptr as usize] = byte;
        self.ram_ptr += 1;
    }

    pub fn update(&mut self) {
        let palette: [u32; 16] = [
            0x000000, 0x1D2B53, 0x7E2553, 0x008751, 0xAB5236, 0x5F574F, 0xC2C3C7, 0xFFF1E8,
            0xFF004D, 0xFFA300, 0xFFEC27, 0x00E436, 0x29ADFF, 0x83769C, 0xFF77A8, 0xFFCCAA,
        ];
        for y in 0..TILES_TALL as usize {
            for x in 0..TILES_WIDE as usize {
                self.render_tile(x, y, &palette);
            }
        }
    }

    pub fn render_tile(&mut self, tile_x: usize, tile_y: usize, palette: &[u32; 16]) {
        let tile_offset = (tile_y * TILES_WIDE) + tile_x;
        let background_address = BACKGROUND_TABLE + (tile_offset * 2);
        let mut tile_address = (self.ram[background_address + 1] as usize) << 8
            | self.ram[background_address] as usize;
        let mut framebuffer_x = tile_x * TILE_WIDTH;
        let mut framebuffer_y = tile_y * TILE_HEIGHT;
        for _ in 0..TILE_SIZE {
            for pixel in 0..2 {
                let (tile_mask, tile_shift, x_offset) = if pixel % 2 == 0 {
                    (0xF0, 4, 0)
                } else {
                    (0x0F, 0, 1)
                };
                let color = palette[((self.ram[tile_address] as usize) & tile_mask) >> tile_shift];
                let color_bytes = u32::to_le_bytes(color);
                let framebuffer_address = ((framebuffer_y * WIDTH) + framebuffer_x + x_offset) * 4;
                self.background[framebuffer_address + 3] = 0xFF;
                self.background[framebuffer_address + 2] = color_bytes[0];
                self.background[framebuffer_address + 1] = color_bytes[1];
                self.background[framebuffer_address] = color_bytes[2];
            }
            framebuffer_x += 2;
            if framebuffer_x % 8 == 0 {
                framebuffer_x -= 8;
                framebuffer_y += 1;
            }
            tile_address += 1;
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let i = i * 4;
            let slice = &self.background[i..i + 4];
            pixel.copy_from_slice(slice);
        }
    }
}
