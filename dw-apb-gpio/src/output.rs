use core::convert::Infallible;

use embedded_hal::digital::{ErrorType, OutputPin, PinState, StatefulOutputPin};

use crate::{
    eint::EintPad,
    eint_with_both_edges::EintPadWithBothEdges,
    input::Input,
    ops,
    pad::{Pad, Restore},
    registers::DwApbGpio,
};

/// Output mode GPIO pad.
pub struct Output<'a> {
    pub(crate) pad: Pad<'a>,
}

impl<'a> Output<'a> {
    /// Acquires and configures an output pad with the requested initial level.
    ///
    /// # Safety
    /// The requirements of [`crate::flex_pad::FlexPad::new`] apply.
    #[inline]
    pub unsafe fn new(port: char, number: u8, gpio: &'a DwApbGpio, state: PinState) -> Self {
        let result = Self {
            pad: Pad::new(port, number, gpio),
        };
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_set_output(cs, &result.pad, state),
        );
        result
    }

    /// Configures the pad as an input.
    #[inline(always)]
    pub fn into_input(self) -> Input<'a> {
        let result = Input { pad: self.pad };
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_set_input(cs, &result.pad),
        );
        result
    }

    /// Temporarily borrows the pad as an input.
    pub fn with_input<R>(&mut self, f: impl for<'b> FnOnce(&mut Input<'b>) -> R) -> R {
        let _restore = Restore::new(&self.pad);
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_set_input(cs, &self.pad),
        );
        f(&mut Input {
            pad: self.pad.reborrow(),
        })
    }
}

impl<'a> Output<'a> {
    /// Configures a port A interrupt input, initially disabled and masked.
    ///
    /// # Safety
    /// The requirements of [`crate::flex_pad::FlexPad::into_eint`] apply.
    #[inline(always)]
    pub unsafe fn into_eint(self) -> EintPad<'a> {
        let result = EintPad { pad: self.pad };
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_into_eint(cs, &result.pad),
        );
        result
    }

    /// Configures a port A interrupt input with hardware both-edge support, initially masked and disabled.
    ///
    /// # Safety
    /// The requirements of [`crate::flex_pad::FlexPad::into_eint_with_both_edges`] apply.
    #[inline(always)]
    pub unsafe fn into_eint_with_both_edges(self) -> EintPadWithBothEdges<'a> {
        let result = EintPadWithBothEdges { pad: self.pad };
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_into_eint(cs, &result.pad),
        );
        result
    }
}

impl Output<'_> {
    /// Sets the output latch low.
    #[inline(always)]
    pub fn set_low(&mut self) {
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_set_level(cs, &self.pad, PinState::Low),
        );
    }

    /// Sets the output latch high.
    #[inline(always)]
    pub fn set_high(&mut self) {
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_set_level(cs, &self.pad, PinState::High),
        );
    }

    /// Sets the output latch to the requested level.
    #[inline(always)]
    pub fn set_level(&mut self, state: PinState) {
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_set_level(cs, &self.pad, state),
        );
    }

    /// Returns whether the output latch is high.
    #[inline(always)]
    pub fn is_set_high(&self) -> bool {
        ops::op_latch(&self.pad)
    }

    /// Returns whether the output latch is low.
    #[inline(always)]
    pub fn is_set_low(&self) -> bool {
        !ops::op_latch(&self.pad)
    }

    /// Toggles the output latch.
    #[inline(always)]
    pub fn toggle(&mut self) {
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_toggle(cs, &self.pad),
        );
    }
}

impl ErrorType for Output<'_> {
    type Error = Infallible;
}

impl OutputPin for Output<'_> {
    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_set_level(cs, &self.pad, PinState::Low),
        );
        Ok(())
    }

    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_set_level(cs, &self.pad, PinState::High),
        );
        Ok(())
    }

    #[inline(always)]
    fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_set_level(cs, &self.pad, state),
        );
        Ok(())
    }
}

impl StatefulOutputPin for Output<'_> {
    #[inline(always)]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(ops::op_latch(&self.pad))
    }

    #[inline(always)]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!ops::op_latch(&self.pad))
    }

    #[inline(always)]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_toggle(cs, &self.pad),
        );
        Ok(())
    }
}
