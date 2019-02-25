use core;
use volatile::Volatile;
use bit_field::BitField;

/* Introduction
The System Integration Module (SIM) provides system control and 
chip configuration registers.
*/

#[derive(Clone, Copy)]
pub enum Clock {
	PortA,
    PortB,
    PortC,
    PortD,
    PortE,
}

#[repr(C,packed)]
pub struct Sim {
	// System options registers
	sopt1: Volatile<u32>,			// SOPT1 register
    sopt1_cfg: Volatile<u32>,     // SOPT1 config register

    _pad0: [u32; 1023], // Skips to 4004_8000, 1kb past sopt1 start

	// SOPT Registers 2 - 7
    sopt2: Volatile<u32>,
    _pad1: Volatile<u32>,			// Skips sopt3 (not documented)
    sopt4: Volatile<u32>,
    sopt5: Volatile<u32>,
    _pad2: Volatile<u32>,			// Skips sopt6 (not documented)
    sopt7: Volatile<u32>,

    _pad3: [u32; 2],	// Skips to 4004_8024

	// Sytem device ID register
    sdid: Volatile<u32>,

	// System clock gating control registers
    _pad4: [u32; 3],	// Skips SCGC 1-3
    scgc4: Volatile<u32>,
    scgc5: Volatile<u32>,
    scgc6: Volatile<u32>,
    scgc7: Volatile<u32>,

    // System clock divider registers
    clkdiv1: Volatile<u32>,
    clkviv2: Volatile<u32>,

    // Flash config register 1
    fcfg1: Volatile<u32>,
    fcfg2: Volatile<u32>,

	// UUID register (should be u128)
    uidh: Volatile<u32>,
    uidmh: Volatile<u32>,
    uidml: Volatile<u32>,
    uidl: Volatile<u32>
}
	
impl Sim {
	// Creates a static System Integration Module at the proper hardware address
    pub unsafe fn new() -> &'static mut Sim {
        &mut *(0x4004_7000 as *mut Sim)
    }

    pub fn enable_clock(&mut self, clock: Clock) {
        unsafe {
            match clock {
                // See section 12.2.12, System Clock Gating Control Register 5
                Clock::PortA => { self.scgc5.update(|scgc| { scgc.set_bit(09, true); }); }
                Clock::PortB => { self.scgc5.update(|scgc| { scgc.set_bit(10, true); }); }
                Clock::PortC => { self.scgc5.update(|scgc| { scgc.set_bit(11, true); }); }
                Clock::PortD => { self.scgc5.update(|scgc| { scgc.set_bit(12, true); }); }
                Clock::PortE => { self.scgc5.update(|scgc| { scgc.set_bit(13, true); }); }
            }
        }
    }

    pub unsafe fn set_dividers(&mut self, core: u32, bus: u32, flash: u32) {
        let mut clkdiv: u32 = 0;
        clkdiv.set_bits(28..32, core-1);
        clkdiv.set_bits(24..28, bus-1);
        clkdiv.set_bits(16..20, flash-1);
        self.clkdiv1.write(clkdiv);
    }
}
