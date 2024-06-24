// clock.rs

use core::num::{NonZeroU8, NonZeroU16};
use core::ops::{Deref, DerefMut};
use core::num::Wrapping;
use z80emu::host::{cycles, Clock};
use cycles::*;

pub type Ts = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TClock {
    current: Wrapping<Ts>,
    clock_hz: Ts,
}

impl TClock {
    pub fn new(clock_hz: Ts) -> Self {
        TClock { current: Wrapping(0), clock_hz }
    }

    pub fn reset(&mut self) {
        self.current = Wrapping(0);
    }

    // CPU clock in t-states / second
    pub fn clock_hz(&self) -> Ts {
        self.clock_hz
    }

    // nanoseconds / t-state
    pub fn ts_duration_nanos(&self) -> Ts {
        1e9 as Ts / self.clock_hz
    }

    pub fn check_wrap_second(&mut self) -> bool {
        if self.current.0 > 2*self.clock_hz {
            self.current -= Wrapping(self.clock_hz);
            true
        }
        else {
            false
        }
    }
}

impl Clock for TClock {
    type Limit = Ts;
    type Timestamp = Ts;

    #[inline]
    fn is_past_limit(&self, limit: Self::Limit) -> bool {
        self.current.0 >= limit
    }

    #[inline]
    fn add_irq(&mut self, _addr: u16) -> Ts {
        self.current += Wrapping(IRQ_ACK_CYCLE_TS.into());
        self.current.0
    }

    #[inline]
    fn add_no_mreq(&mut self, _addr: u16, add_ts: NonZeroU8) {
        self.current += Wrapping(add_ts.get().into());
    }

    #[inline]
    fn add_io(&mut self, _port: u16) -> Ts {
        self.current += Wrapping(IO_CYCLE_TS.into());
        self.current.0
    }

    #[inline]
    fn add_mreq(&mut self, _addr: u16) -> Ts {
        self.current += Wrapping(MEMRW_CYCLE_TS.into());
        self.current.0
    }

    #[inline]
    fn add_m1(&mut self, _addr: u16) -> Ts {
        self.current += Wrapping(M1_CYCLE_TS.into());
        self.current.0
    }

    #[inline]
    fn add_wait_states(&mut self, _bus: u16, wait_states: NonZeroU16) {
        self.current += Wrapping(wait_states.get().into());
    }

    #[inline]
    fn as_timestamp(&self) -> Ts {
        self.current.0
    }
}

impl Deref for TClock {
    type Target = Wrapping<Ts>;

    fn deref(&self) -> &Self::Target {
        &self.current
    }
}

impl DerefMut for TClock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.current
    }
}
