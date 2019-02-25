
use core;
use super::port::*;
use super::port::PortMuxCtrl;


pub struct Pin {
	pub port: *mut PortMuxCtrl, // subtly suggested safety 8)
	pub pin: usize // pin #, 0-31
}



pub struct GpioPin {
	gpio: *mut GpioPortBitband,
	pin: Pin
}

impl Pin {
	pub fn mode_gpio(self) -> GpioPin {
		unsafe {
			let port = &mut *self.port;
			port.set_pin_mode(self.pin, 1);
			GpioPin::new(port.name(), self.pin)
		}
	}
}

impl GpioPin {
	pub unsafe fn new(port: PortName, pin: usize) -> GpioPin {
		let gpio = match port {
			PortName::A => 0x43FE_0000 as *mut GpioPortBitband,
			PortName::B => 0x43FE_0800 as *mut GpioPortBitband,
			PortName::C => 0x43FE_1000 as *mut GpioPortBitband,
			PortName::D => 0x43FE_1800 as *mut GpioPortBitband,
			PortName::E => 0x43FE_2000 as *mut GpioPortBitband
		};

		GpioPin {
			gpio: gpio,
			pin: Pin {
				port: (PortMuxCtrl::new(port)),
				pin
			}
		}
	}

	pub fn input(&mut self) {
		unsafe {
			(*self.gpio).pddr[self.pin.pin].write(0)
		}
	}

	pub fn output(&mut self) {
		unsafe {
			(*self.gpio).pddr[self.pin.pin].write(1);
		}
	}

	pub fn high(&mut self) {
		unsafe {
			(*self.gpio).psor[self.pin.pin].write(1);
		}
	}

	pub fn low(&mut self) {
		unsafe {
			(*self.gpio).pcor[self.pin.pin].write(1);
		}
	}

	pub fn read(&mut self) -> bool {
		unsafe {
			(*self.gpio).pdir[self.pin.pin].read() == 1
		}
	}

	pub fn pull_down(&mut self) {
		unsafe {
			(*(*self).pin.port).set_pin_pe((*self).pin.pin, true);
			(*(*self).pin.port).set_pin_ps((*self).pin.pin, false)
		}
	}

	pub fn pull_up(&mut self) {
		unsafe {
			(*(*self).pin.port).set_pin_pe((*self).pin.pin, true);
			(*(*self).pin.port).set_pin_ps((*self).pin.pin, true)
		}
	}
}