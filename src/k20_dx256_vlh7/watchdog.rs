use volatile::Volatile;
use bit_field::BitField;
use core::arch::arm::__NOP;

/* Introduction
The Watchdog Timer (WDOG) keeps a watch on the system functioning and resets
it in case of its failure. Reasons for failure include run-away software code
and the stoppage of the system clock that in a safety critical system can lead
to serious consequences. In such cases, the watchdog brings the system into a
safe state of operation. The watchdog monitors the operation of the system by
expecting periodic communication from the software, generally known as
servicing or refreshing the watchdog. If this periodic refreshing does not
occur, the watchdog resets the system
*/



impl Watchdog {

    // Creates a static watchdog at the proper 32 bit address: 0x4005_2000
    pub unsafe fn new() -> &'static mut Watchdog {
        &mut *(0x4005_2000 as *mut Watchdog)
    }

    pub unsafe fn unlock(&mut self) {
        // Writing the unlock sequence values to this register
        // makes the watchdog write-once registers writable again.

        // The required unlock sequence is 0xC520 followed by 0xD928
        // within 20 bus clock cycles.

        // A valid unlock sequence opens a window equal in length to the
        // WCT within which you can update the registers.

        // The unlock sequence is effective only if ALLOWUPDATE is set.
        // By default, ALLOWUPDATE is set to 1.
        self.unlock.write(0xC520);
        self.unlock.write(0xD928);
        // Wait 2 cycles for the system to update to the fast-clock-input
        __NOP();
        __NOP();
    }

    pub fn disable(&mut self) {
        unsafe {
            self.unlock();
            // The last bit ofstctrlh is WDOG_EN (Watchdog Enable),
            // which enables or disables watchdog's operation.
            self.stctrlh.update(|ctrl| {
                ctrl.set_bit(0, false);
            });
        }
    }
}

#[repr(C,packed)]
pub struct Watchdog {
	// These are registers in the __software__ watchdog
	// Some 32 bit registers are split into high and low 16 bit registers

	stctrlh: Volatile<u16>, stctrll: Volatile<u16>, // Status & Ctrl register(s)
    tovalh: Volatile<u16>, tovall: Volatile<u16>,   // Time-out value register(s)
    winh: Volatile<u16>, winl: Volatile<u16>,       // Window register(s)
    refresh: Volatile<u16>,               // Refresh register
    unlock: Volatile<u16>,                // Unlock register
    tmrouth: Volatile<u16>, tmroutl: Volatile<u16>, // Timer output register(s)
    rstcnt: Volatile<u16>,                // Reset count
    presc: Volatile<u16>                  //   Prescaler
}
