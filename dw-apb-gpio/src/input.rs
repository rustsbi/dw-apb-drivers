use core::convert::Infallible;

use embedded_hal::digital::{ErrorType, InputPin, PinState};

use crate::{
    eint::EintPad,
    eint_with_both_edges::EintPadWithBothEdges,
    ops,
    output::Output,
    pad::{Pad, Restore},
    registers::DwApbGpio,
};

/// Input mode GPIO pad.
pub struct Input<'a> {
    pub(crate) pad: Pad<'a>,
}

impl<'a> Input<'a> {
    /// Acquires and configures an input pad.
    ///
    /// # Safety
    /// The requirements of [`crate::flex_pad::FlexPad::new`] apply.
    #[inline]
    pub unsafe fn new(port: char, number: u8, gpio: &'a DwApbGpio) -> Self {
        let result = Self {
            pad: Pad::new(port, number, gpio),
        };
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_set_input(cs, &result.pad),
        );
        result
    }

    /// Configures the pad as an output with the requested initial level.
    #[inline(always)]
    pub fn into_output(self, state: PinState) -> Output<'a> {
        let result = Output { pad: self.pad };
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_set_output(cs, &result.pad, state),
        );
        result
    }

    /// Temporarily borrows the pad as an output with the requested initial level.
    pub fn with_output<R>(
        &mut self,
        state: PinState,
        f: impl for<'b> FnOnce(&mut Output<'b>) -> R,
    ) -> R {
        let _restore = Restore::new(&self.pad);
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_set_output(cs, &self.pad, state),
        );
        f(&mut Output {
            pad: self.pad.reborrow(),
        })
    }
}

impl<'a> Input<'a> {
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

impl Input<'_> {
    /// Returns whether the external pin level is high.
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        ops::op_level(&self.pad)
    }

    /// Returns whether the external pin level is low.
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        !ops::op_level(&self.pad)
    }
}

impl ErrorType for Input<'_> {
    type Error = Infallible;
}

impl InputPin for Input<'_> {
    #[inline(always)]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(ops::op_level(&self.pad))
    }

    #[inline(always)]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!ops::op_level(&self.pad))
    }
}
