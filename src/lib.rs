//#![no_std]

use core::ptr::{read_volatile, write_volatile};
use core::marker::PhantomData;

//use embedded_hal::digital::v2::{OutputPin};

mod adns_2620;

/*
class OptiMouse
{
  private:
  protected:
	uint8_t _sclkPin;
    uint8_t _sdioPin;
	uint8_t readRegister(uint8_t);
	void writeRegister(uint8_t, uint8_t);
  public:
    OptiMouse(uint8_t, uint8_t);
    void begin(void);
	signed char dx(void);
	signed char dy(void);
};
*/

/*
pub trait Active {}

pub struct Input<MODE: ?Sized> {
    _mode: PhantomData<MODE>,
}

impl<MODE> Active for Input<MODE> {}

pub struct Output<MODE: ?Sized> {
    _mode: PhantomData<MODE>,
}

impl<MODE> Active for Output<MODE> {}

pub struct Pin<MODE>
    where MODE: Active + Sized {
    address: *const usize,
 l  value: usize,
    _mode: PhantomData<MODE>,
}

impl<MODE: Active> Active for Pin<MODE> {}

impl<MODE: Active> Pin<MODE> {
    fn read(&self) -> usize {
        unsafe { read_volatile(self.address) }
    }

    fn write(&self, value: usize) {
        unsafe { write_volatile(self.address as *mut usize, value); }
    }
}

pub struct OpticMouse {
    sclk_pin: Pin<Output<Active>>,
    sdio_pin: Pin<Input<Active>>,
}
*/

pub struct OpticMouse {
    sclk_pin: *const usize,
    sdio_pin: *const usize,
}

impl OpticMouse {
    pub fn new(sclk_pin: *const usize, sdio_pin: *const usize) -> Self {
        OpticMouse {
            sclk_pin,
            sdio_pin,
        }
    }

    pub fn read_clk(&self) -> usize {
        unsafe { read_volatile(self.sclk_pin) }
    }

    pub fn read_dio(&self) -> usize {
        unsafe { read_volatile(self.sdio_pin) }
    }

    pub fn write_clk(&self, val: usize) {
        unsafe { write_volatile(self.sclk_pin as *mut usize, val); }
    }
}

#[cfg(test)]
mod tests {
    use crate::OpticMouse;

    #[test]
    fn reading() {

        let (a, b) = (42, 30);
        let m = OpticMouse::new(&a, &b);

        m.write_clk(b);
        println!("{}", m.read_clk());
        assert_eq!(m.read_clk(), b);

    }

    #[test]
    fn adns() {
        use crate::adns_2620::*;
        let mouse = Adns2620::new();

        assert_eq!(mouse.delta_x(), 0);
    }
}
