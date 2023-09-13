#![cfg_attr(not(test), no_std)]

use embedded_hal::digital::v2::OutputPin;

fn digits(value: u8) -> (u8, u8) {
    let lo = value & 0x0F;
    let hi = (value & 0xF0) >> 4;
    (hi, lo)
}
pub struct SevenSeg<A, B, C, D, E, F, G, U, T> {
    seg_a: A,
    seg_b: B,
    seg_c: C,
    seg_d: D,
    seg_e: E,
    seg_f: F,
    seg_g: G,
    en_u: U,
    en_t: T,
    active_state: bool,
}

impl<A, B, C, D, E, F, G, U, T> SevenSeg<A, B, C, D, E, F, G, U, T>
where
    A: OutputPin,
    B: OutputPin,
    C: OutputPin,
    D: OutputPin,
    E: OutputPin,
    F: OutputPin,
    G: OutputPin,
    U: OutputPin,
    T: OutputPin,
{
    pub fn new(
        seg_a: A,
        seg_b: B,
        seg_c: C,
        seg_d: D,
        seg_e: E,
        seg_f: F,
        seg_g: G,
        en_u: U,
        en_t: T,
        active_state: bool,
    ) -> Self {
        Self {
            seg_a,
            seg_b,
            seg_c,
            seg_d,
            seg_e,
            seg_f,
            seg_g,
            en_u,
            en_t,
            active_state,
        }
    }

    fn state(&self, value: bool) -> bool {
        return value ^ !self.active_state;
    }

    /// Disable the 7-segment display by pulling all GPIOs low
    pub fn clear(&mut self) -> Result<(), ()> {
        self.choose_segment(0)?;

        self.seg_a(false)?;
        self.seg_b(false)?;
        self.seg_c(false)?;
        self.seg_d(false)?;
        self.seg_e(false)?;
        self.seg_f(false)?;
        self.seg_g(false)
    }

    /// Enable or disable segment `a` according to the `state`
    pub fn seg_a(&mut self, value: bool) -> Result<(), ()> {
        if self.state(value) {
            self.seg_a.set_high().map_err(|_| ())
        } else {
            self.seg_a.set_low().map_err(|_| ())
        }
    }

    /// Enable or disable segment `b` according to the `state`
    pub fn seg_b(&mut self, value: bool) -> Result<(), ()> {
        if self.state(value) {
            self.seg_b.set_high().map_err(|_| ())
        } else {
            self.seg_b.set_low().map_err(|_| ())
        }
    }

    /// Enable or disable segment `c` according to the `state`
    pub fn seg_c(&mut self, value: bool) -> Result<(), ()> {
        if self.state(value) {
            self.seg_c.set_high().map_err(|_| ())
        } else {
            self.seg_c.set_low().map_err(|_| ())
        }
    }

    /// Enable or disable segment `d` according to the `state`
    pub fn seg_d(&mut self, value: bool) -> Result<(), ()> {
        if self.state(value) {
            self.seg_d.set_high().map_err(|_| ())
        } else {
            self.seg_d.set_low().map_err(|_| ())
        }
    }

    /// Enable or disable segment `e` according to the `state`
    pub fn seg_e(&mut self, value: bool) -> Result<(), ()> {
        if self.state(value) {
            self.seg_e.set_high().map_err(|_| ())
        } else {
            self.seg_e.set_low().map_err(|_| ())
        }
    }

    /// Enable or disable segment `f` according to the `state`
    pub fn seg_f(&mut self, value: bool) -> Result<(), ()> {
        if self.state(value) {
            self.seg_f.set_high().map_err(|_| ())
        } else {
            self.seg_f.set_low().map_err(|_| ())
        }
    }

    /// Enable or disable segment `g` according to the `state`
    pub fn seg_g(&mut self, value: bool) -> Result<(), ()> {
        if self.state(value) {
            self.seg_g.set_high().map_err(|_| ())
        } else {
            self.seg_g.set_low().map_err(|_| ())
        }
    }

    fn set_value(&mut self, value: u8) -> Result<(), ()> {
        match value {
            0 => {
                self.seg_a(true)?;
                self.seg_b(true)?;
                self.seg_c(true)?;
                self.seg_d(true)?;
                self.seg_e(true)?;
                self.seg_f(true)?;
                self.seg_g(false)
            }
            1 => {
                self.seg_a(false)?;
                self.seg_b(true)?;
                self.seg_c(true)?;
                self.seg_d(false)?;
                self.seg_e(false)?;
                self.seg_f(false)?;
                self.seg_g(false)
            }
            2 => {
                self.seg_a(true)?;
                self.seg_b(true)?;
                self.seg_c(false)?;
                self.seg_d(true)?;
                self.seg_e(true)?;
                self.seg_f(false)?;
                self.seg_g(true)
            }
            3 => {
                self.seg_a(true)?;
                self.seg_b(true)?;
                self.seg_c(true)?;
                self.seg_d(true)?;
                self.seg_e(false)?;
                self.seg_f(false)?;
                self.seg_g(true)
            }
            4 => {
                self.seg_a(false)?;
                self.seg_b(true)?;
                self.seg_c(true)?;
                self.seg_d(false)?;
                self.seg_e(false)?;
                self.seg_f(true)?;
                self.seg_g(true)
            }
            5 => {
                self.seg_a(true)?;
                self.seg_b(false)?;
                self.seg_c(true)?;
                self.seg_d(true)?;
                self.seg_e(false)?;
                self.seg_f(true)?;
                self.seg_g(true)
            }
            6 => {
                self.seg_a(true)?;
                self.seg_b(false)?;
                self.seg_c(true)?;
                self.seg_d(true)?;
                self.seg_e(true)?;
                self.seg_f(true)?;
                self.seg_g(true)
            }
            7 => {
                self.seg_a(true)?;
                self.seg_b(true)?;
                self.seg_c(true)?;
                self.seg_d(false)?;
                self.seg_e(false)?;
                self.seg_f(false)?;
                self.seg_g(false)
            }
            8 => {
                self.seg_a(true)?;
                self.seg_b(true)?;
                self.seg_c(true)?;
                self.seg_d(true)?;
                self.seg_e(true)?;
                self.seg_f(true)?;
                self.seg_g(true)
            }
            9 => {
                self.seg_a(true)?;
                self.seg_b(true)?;
                self.seg_c(true)?;
                self.seg_d(true)?;
                self.seg_e(false)?;
                self.seg_f(true)?;
                self.seg_g(true)
            }
            0x0A => {
                self.seg_a(true)?;
                self.seg_b(true)?;
                self.seg_c(true)?;
                self.seg_d(false)?;
                self.seg_e(true)?;
                self.seg_f(true)?;
                self.seg_g(true)
            }
            0x0B => {
                self.seg_a(false)?;
                self.seg_b(false)?;
                self.seg_c(true)?;
                self.seg_d(true)?;
                self.seg_e(true)?;
                self.seg_f(true)?;
                self.seg_g(true)
            }
            0x0C => {
                self.seg_a(true)?;
                self.seg_b(false)?;
                self.seg_c(false)?;
                self.seg_d(true)?;
                self.seg_e(true)?;
                self.seg_f(true)?;
                self.seg_g(false)
            }
            0x0D => {
                self.seg_a(false)?;
                self.seg_b(true)?;
                self.seg_c(true)?;
                self.seg_d(true)?;
                self.seg_e(true)?;
                self.seg_f(false)?;
                self.seg_g(true)
            }
            0x0E => {
                self.seg_a(true)?;
                self.seg_b(false)?;
                self.seg_c(false)?;
                self.seg_d(true)?;
                self.seg_e(true)?;
                self.seg_f(true)?;
                self.seg_g(true)
            }
            0x0F => {
                self.seg_a(true)?;
                self.seg_b(false)?;
                self.seg_c(false)?;
                self.seg_d(false)?;
                self.seg_e(true)?;
                self.seg_f(true)?;
                self.seg_g(true)
            }
            0xFF => {
                self.seg_a(false)?;
                self.seg_b(false)?;
                self.seg_c(false)?;
                self.seg_d(false)?;
                self.seg_e(false)?;
                self.seg_f(false)?;
                self.seg_g(false)
            }
            _ => self.clear(),
        }
    }

    //NOTE:
    // 0 - all
    // 1 - unity
    // 2 - tens
    fn choose_segment(&mut self, segment: u8) -> Result<(), ()> {
        match segment {
            2 => {
                self.en_t.set_low().map_err(|_| ())?;
                self.en_u.set_high().map_err(|_| ())?;
            }
            1 => {
                self.en_u.set_low().map_err(|_| ())?;
                self.en_t.set_high().map_err(|_| ())?;
            }
            0 => {
                self.en_u.set_low().map_err(|_| ())?;
                self.en_t.set_low().map_err(|_| ())?;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn display(&mut self, value: u8, disp_time_flag: u8) -> Result<(), ()> {
        let (hi, lo) = digits(value);

        match disp_time_flag {
            1 => {
                self.set_value(0xFF)?;
                self.choose_segment(1)?;
                self.set_value(lo)?;
            }
            2 => {
                self.set_value(0xFF)?;
                self.choose_segment(2)?;
                self.set_value(hi)?;
            }
            _ => {
                self.set_value(0xFF)?;
                self.choose_segment(0)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use embedded_hal_mock::pin::{
        Mock as PinMock, State as PinState, Transaction as PinTransaction,
    };

    #[test]
    fn segment_test() {
        let expect_high = [
            PinTransaction::set(PinState::High),
            PinTransaction::get(PinState::High),
        ];
        let expect_low = [
            PinTransaction::get(PinState::High),
            PinTransaction::set(PinState::Low),
            PinTransaction::get(PinState::Low),
        ];

        //let mut pin = PinMock::new(&expectations);
        let seg_a = PinMock::new(&expect_high);
        let seg_b = PinMock::new(&expect_low);
        let seg_c = PinMock::new(&expect_low);
        let seg_d = PinMock::new(&expect_high);
        let seg_e = PinMock::new(&expect_high);
        let seg_f = PinMock::new(&expect_low);
        let seg_g = PinMock::new(&expect_high);
        let en_u = PinMock::new(&expect_low);
        let en_t = PinMock::new(&expect_high);
        let active_state: bool = true;
        let mut display = SevenSeg::new(
            seg_a,
            seg_b,
            seg_c,
            seg_d,
            seg_e,
            seg_f,
            seg_g,
            en_u,
            en_t,
            active_state,
        );
        assert_eq!(SevenSeg::seg_a(&mut display, true), Ok(()));
        assert_eq!(SevenSeg::seg_d(&mut display, false), Err(()));
    }

    #[test]
    fn value_test() {
        let expect_high = [
            PinTransaction::set(PinState::High),
            PinTransaction::get(PinState::High),
        ];
        let expect_low = [
            PinTransaction::set(PinState::Low),
            PinTransaction::get(PinState::Low),
        ];
        let seg_a = PinMock::new(&expect_low);
        let seg_b = PinMock::new(&expect_high);
        let seg_c = PinMock::new(&expect_high);
        let seg_d = PinMock::new(&expect_low);
        let seg_e = PinMock::new(&expect_low);
        let seg_f = PinMock::new(&expect_low);
        let seg_g = PinMock::new(&expect_low);
        let en_u = PinMock::new(&expect_low);
        let en_t = PinMock::new(&expect_low);
        let active_state = true;
        let mut display = SevenSeg::new(
            seg_a,
            seg_b,
            seg_c,
            seg_d,
            seg_e,
            seg_f,
            seg_g,
            en_u,
            en_t,
            active_state,
        );
        assert_eq!(display.set_value(1), Ok(()));
    }
    
    #[test]
    fn choice_test() {
        let expect_high = [
            PinTransaction::set(PinState::High),
            PinTransaction::get(PinState::High),
        ];
        let expect_low = [
            PinTransaction::set(PinState::Low),
            PinTransaction::get(PinState::Low),
        ];
        let seg_a = PinMock::new(&expect_low);
        let seg_b = PinMock::new(&expect_high);
        let seg_c = PinMock::new(&expect_high);
        let seg_d = PinMock::new(&expect_low);
        let seg_e = PinMock::new(&expect_low);
        let seg_f = PinMock::new(&expect_low);
        let seg_g = PinMock::new(&expect_low);
        let en_u = PinMock::new(&expect_high);
        let en_t = PinMock::new(&expect_low);
        let active_state = true;
        let mut display = SevenSeg::new(
            seg_a,
            seg_b,
            seg_c,
            seg_d,
            seg_e,
            seg_f,
            seg_g,
            en_u,
            en_t,
            active_state,
        );
        assert_eq!(display.choose_segment(2), Ok(()));
        display.choose_segment(7).expect_err("Invalid segment");
    }
 }
