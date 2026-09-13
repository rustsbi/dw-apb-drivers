use core::convert::Infallible;

use embedded_hal::digital::{ErrorType, InputPin, PinState};

use crate::{input::Input, ops, output::Output, pad::Pad, registers::DwApbGpio};

/// External interrupt mode pad on port A with hardware both-edge detection.
pub struct EintPadWithBothEdges<'a> {
    pub(crate) pad: Pad<'a>,
}

/// External interrupt event including hardware both-edge detection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventWithBothEdges {
    /// Rising edge.
    RisingEdge,
    /// Falling edge.
    FallingEdge,
    /// High level.
    HighLevel,
    /// Low level.
    LowLevel,
    /// Rising and falling edges.
    BothEdges,
}

impl<'a> EintPadWithBothEdges<'a> {
    /// Acquires a port A interrupt input, initially disabled and masked.
    ///
    /// # Safety
    /// The requirements of [`crate::flex_pad::FlexPad::new`] and [`crate::flex_pad::FlexPad::into_eint_with_both_edges`] apply.
    #[inline]
    pub unsafe fn new(number: u8, gpio: &'a DwApbGpio) -> Self {
        let result = Self {
            pad: Pad::new('A', number, gpio),
        };
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_into_eint(cs, &result.pad),
        );
        result
    }

    /// Selects an interrupt event.
    #[inline(always)]
    pub fn set_event(&mut self, event: EventWithBothEdges) {
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_set_event_with_both_edges(cs, &self.pad, event),
        );
    }

    /// Enables and unmasks the pad's interrupt.
    #[inline(always)]
    pub fn enable_interrupt(&mut self) {
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_enable_interrupt(cs, &self.pad),
        );
    }

    /// Masks and disables the pad's interrupt.
    #[inline(always)]
    pub fn disable_interrupt(&mut self) {
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_disable_interrupt(cs, &self.pad),
        );
    }

    /// Clears a pending edge interrupt; an active level interrupt remains pending.
    #[inline(always)]
    pub fn clear_interrupt(&mut self) {
        ops::op_clear_interrupt(&self.pad);
    }

    /// Returns whether the pad has an unmasked pending interrupt.
    #[inline(always)]
    pub fn is_interrupt_pending(&self) -> bool {
        ops::op_interrupt_pending(&self.pad)
    }

    /// Disables the interrupt and returns the pad as an input.
    #[inline(always)]
    pub fn into_input(self) -> Input<'a> {
        let result = Input { pad: self.pad };
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_disable_interrupt(cs, &result.pad),
        );
        result
    }

    /// Disables the interrupt and configures an output with the requested initial level.
    #[inline(always)]
    pub fn into_output(self, state: PinState) -> Output<'a> {
        let result = Output { pad: self.pad };
        critical_section::with(
            #[inline(always)]
            |cs| ops::op_eint_into_output(cs, &result.pad, state),
        );
        result
    }
}

impl EintPadWithBothEdges<'_> {
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

impl ErrorType for EintPadWithBothEdges<'_> {
    type Error = Infallible;
}

impl InputPin for EintPadWithBothEdges<'_> {
    #[inline(always)]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(ops::op_level(&self.pad))
    }

    #[inline(always)]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!ops::op_level(&self.pad))
    }
}
