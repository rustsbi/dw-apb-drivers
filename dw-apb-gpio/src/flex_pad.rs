use core::convert::Infallible;

use embedded_hal::digital::{ErrorType, InputPin, OutputPin, PinState, StatefulOutputPin};

use crate::{
    eint::EintPad, eint_with_both_edges::EintPadWithBothEdges, input::Input, ops, output::Output,
    pad::Pad, registers::DwApbGpio,
};

/// Flexible GPIO pad.
pub struct FlexPad<'a> {
    pub(crate) pad: Pad<'a>,
}

impl<'a> FlexPad<'a> {
    /// Acquires a pad on port A, B, C or D without changing its direction.
    ///
    /// # Safety
    /// The pin must exist, be exclusively owned, and remain in software GPIO mode;
    /// its interrupt must initially be disabled. Its registers and clocks must remain
    /// accessible for `'a`. All shared-register writers must use the same critical section,
    /// whose implementation must exclude concurrent interrupts and other cores.
    #[inline]
    pub unsafe fn new(port: char, number: u8, gpio: &'a DwApbGpio) -> Self {
        Self {
            pad: Pad::new(port, number, gpio),
        }
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

    /// Configures the pad as an input while retaining its flexible type.
    #[inline(always)]
    pub fn set_as_input(&mut self) {
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_set_input(cs, &self.pad),
        );
    }

    /// Configures the pad as an output with the requested initial level.
    #[inline(always)]
    pub fn set_as_output(&mut self, state: PinState) {
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_set_output(cs, &self.pad, state),
        );
    }

    /// Configures a port A input for interrupts, initially disabled and masked.
    ///
    /// # Safety
    /// Port A interrupt registers must be implemented without hardware both-edge detection.
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
    /// Port A interrupt registers must be implemented including the hardware both-edge register.
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

impl<'a> From<Input<'a>> for FlexPad<'a> {
    #[inline(always)]
    fn from(value: Input<'a>) -> Self {
        Self { pad: value.pad }
    }
}

impl<'a> From<Output<'a>> for FlexPad<'a> {
    #[inline(always)]
    fn from(value: Output<'a>) -> Self {
        Self { pad: value.pad }
    }
}

impl<'a> From<EintPad<'a>> for FlexPad<'a> {
    #[inline(always)]
    fn from(value: EintPad<'a>) -> Self {
        let result = Self { pad: value.pad };
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_disable_interrupt(cs, &result.pad),
        );
        result
    }
}

impl<'a> From<EintPadWithBothEdges<'a>> for FlexPad<'a> {
    #[inline(always)]
    fn from(value: EintPadWithBothEdges<'a>) -> Self {
        let result = Self { pad: value.pad };
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_disable_interrupt(cs, &result.pad),
        );
        result
    }
}

impl FlexPad<'_> {
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

impl ErrorType for FlexPad<'_> {
    type Error = Infallible;
}

impl InputPin for FlexPad<'_> {
    #[inline(always)]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(ops::op_level(&self.pad))
    }

    #[inline(always)]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!ops::op_level(&self.pad))
    }
}

impl OutputPin for FlexPad<'_> {
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

impl StatefulOutputPin for FlexPad<'_> {
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
