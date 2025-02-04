// memory.rs

pub type MemoryCart = [u8; 32 * 1024];
pub type MemoryRam = [u8; 64 * 1024];

pub struct Memory {
    pub cart: MemoryCart,
    pub cart_enabled: bool,
    pub ram: MemoryRam,
}

impl Memory {
    pub fn new(cart: MemoryCart, ram: MemoryRam) -> Self {
        Memory {
            cart,
            cart_enabled: true,
            ram,
        }
    }

    pub fn read_memory(&self, address: u16) -> u8 {
        let address = address as usize;
        if address < 0x8000 && self.cart_enabled {
            self.cart[address]
        } else {
            self.ram[address]
        }
    }

    pub fn write_memory(&mut self, address: u16, byte: u8) {
        self.ram[address as usize] = byte;
    }
}
