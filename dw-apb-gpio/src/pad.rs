use crate::{ops, registers::DwApbGpio};

pub(crate) struct Pad<'a> {
    pub(crate) port: u8,
    pub(crate) mask: u32,
    pub(crate) gpio: &'a DwApbGpio,
}

impl<'a> Pad<'a> {
    #[inline]
    pub(crate) fn new(port: char, number: u8, gpio: &'a DwApbGpio) -> Self {
        assert!(matches!(port, 'A'..='D'), "invalid GPIO port");
        assert!(number < 32, "pin is outside the register");
        Self {
            port: port as u8 - b'A',
            mask: 1 << number,
            gpio,
        }
    }

    pub(crate) fn reborrow(&self) -> Pad<'_> {
        Pad {
            port: self.port,
            mask: self.mask,
            gpio: self.gpio,
        }
    }
}

pub(crate) struct Restore<'p, 'a> {
    pad: &'p Pad<'a>,
    // Bit 1 is output direction; bit 0 is a high output latch.
    state: u8,
}

impl<'p, 'a> Restore<'p, 'a> {
    pub(crate) fn new(pad: &'p Pad<'a>) -> Self {
        let state = ops::op_save(pad);
        Self { pad, state }
    }
}

impl Drop for Restore<'_, '_> {
    #[inline(always)]
    fn drop(&mut self) {
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_restore(cs, self.pad, self.state),
        );
    }
}
