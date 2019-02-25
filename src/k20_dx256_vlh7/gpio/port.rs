use volatile::Volatile;
use bit_field::BitField;
use super::pin::*;

#[repr(C,packed)]
pub struct GpioPortBitband {
    // Port registers for I/O ops
    pub pdor: [Volatile<u32>; 32], // data output
    pub psor: [Volatile<u32>; 32], // set output
    pub pcor: [Volatile<u32>; 32], // clear output
    pub ptor: [Volatile<u32>; 32], // toggle output
    pub pdir: [Volatile<u32>; 32], // data input
    pub pddr: [Volatile<u32>; 32]  // data direction
}

impl PortMuxCtrl {
    pub unsafe fn new(name: PortName) -> &'static mut PortMuxCtrl {
        &mut * match name {
            PortName::A => 0x4004_9000 as *mut PortMuxCtrl,
            PortName::B => 0x4004_A000 as *mut PortMuxCtrl,
            PortName::C => 0x4004_B000 as *mut PortMuxCtrl,
            PortName::D => 0x4004_C000 as *mut PortMuxCtrl,
            PortName::E => 0x4004_D000 as *mut PortMuxCtrl,
        }
    }

    pub unsafe fn set_pin_mode(&mut self, p: usize, mode: u32) {
        assert!(p < 32);
        self.pcr[p].update(|pcr| {
            pcr.set_bits(8..11, mode);
        });
    }

    pub unsafe fn set_pin_pe(&mut self, p: usize, mode: bool) {
        assert!(p < 32);
        self.pcr[p].update(|pcr| {
            pcr.set_bit(1, mode);
        });
    }

    pub unsafe fn set_pin_ps(&mut self, p: usize, mode: bool) {
        assert!(p < 32);
        self.pcr[p].update(|pcr| {
            pcr.set_bit(0, mode);
        });
    }

    pub unsafe fn pin(&mut self, p: usize) -> Pin {
        Pin { port: self, pin: p }
    }

    pub fn name(&self) -> PortName {
        let addr = (self as *const PortMuxCtrl) as u32;
        match addr {
            0x4004_9000 => PortName::A,
            0x4004_A000 => PortName::B,
            0x4004_B000 => PortName::C,
            0x4004_C000 => PortName::D,
            0x4004_D000 => PortName::E,
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Copy)]
pub enum PortName {
    A,
    B,
    C,
    D,
    E
}

#[repr(C,packed)]
pub struct PortMuxCtrl {
    pcr: [Volatile<u32>; 32], // Port control registers (0-31)
    // Global Port Control register
    gpclr: Volatile<u32>, gpcvhr: Volatile<u32>,
    reserved_0: [u32; 6],
    isfr: Volatile<u32>,	  // Interrupt status flag register
}