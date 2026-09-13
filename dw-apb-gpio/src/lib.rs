//! Synopsys DesignWare APB GPIO driver and registers.

#![no_std]
#![deny(missing_docs)]

mod eint;
mod eint_with_both_edges;
mod flex_pad;
mod input;
mod ops;
mod output;
mod pad;
pub mod prelude;
mod registers;

pub use eint::{EintPad, Event};
pub use eint_with_both_edges::{EintPadWithBothEdges, EventWithBothEdges};
pub use embedded_hal::digital::PinState;
pub use flex_pad::FlexPad;
pub use input::Input;
pub use output::Output;
pub use registers::{DwApbGpio, LevelSync, PinBits, Port};

#[cfg(test)]
mod tests;
