// bus.rs

use crate::vdp::Vdp;
use crate::Buttons;
use crate::Memory as BusMemory;
use core::num::NonZeroU16;
use std::sync::{Arc, Mutex};
use z80emu::{Io, Memory};

pub struct Bus {
    pub buttons: Arc<Mutex<Buttons>>,
    pub memory: BusMemory,
    pub vdp: Arc<Mutex<Vdp>>,

    pub reset: bool,
}

impl Io for Bus {
    type Timestamp = crate::clock::Ts;
    type WrIoBreak = ();
    type RetiBreak = ();

    #[inline(always)]
    fn write_io(
        &mut self,
        port: u16,
        byte: u8,
        _ts: Self::Timestamp,
    ) -> (Option<()>, Option<NonZeroU16>) {
        let masked_port = port & 0x00FF;

        match masked_port {
            0x00 => self.memory.cart_enabled = byte != 0,

            // VDP port
            0x10 => {
                let mut vdp_lock = self.vdp.lock().unwrap();
                vdp_lock.set_pointer_low(byte);
            }
            0x11 => {
                let mut vdp_lock = self.vdp.lock().unwrap();
                vdp_lock.set_pointer_high(byte);
            }
            0x12 => {
                let mut vdp_lock = self.vdp.lock().unwrap();
                vdp_lock.write_memory(byte);
            }

            _ => println!(
                "[write_io] write to unmapped port: {:#X}, byte: {:#X}",
                masked_port, byte
            ),
        }

        (None, None)
    }

    #[inline(always)]
    fn read_io(&mut self, port: u16, _ts: Self::Timestamp) -> (u8, Option<NonZeroU16>) {
        let masked_port = port & 0x00FF;

        let data = match masked_port {
            0x00 => self.memory.cart_enabled as u8,

            // VDP port
            0x12 => {
                let mut vdp_lock = self.vdp.lock().unwrap();
                vdp_lock.read_memory()
            }

            // buttons port
            0x20 => {
                let buttons_lock = self.buttons.lock().unwrap();
                u8::from(&*buttons_lock)
            }

            _ => {
                println!("[read_io] read from unmapped port: {:#X}", masked_port);
                0
            }
        };

        (data, None)
    }
}

impl Memory for Bus {
    type Timestamp = crate::clock::Ts;

    fn read_mem(&self, address: u16, _ts: Self::Timestamp) -> u8 {
        self.memory.read_memory(address)
    }

    fn read_mem16(&self, address: u16, _ts: Self::Timestamp) -> u16 {
        (self.memory.read_memory(address + 1) as u16) << 8
            | (self.memory.read_memory(address) as u16)
    }

    fn read_opcode(&mut self, pc: u16, _ir: u16, _ts: Self::Timestamp) -> u8 {
        self.memory.read_memory(pc)
    }

    fn write_mem(&mut self, address: u16, byte: u8, _ts: Self::Timestamp) {
        self.memory.write_memory(address, byte);
    }
}
