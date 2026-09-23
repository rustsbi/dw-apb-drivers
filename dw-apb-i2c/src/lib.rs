//! Synopsys DesignWare APB I²C registers.
//!
//! This crate models registers, not a transaction driver. The caller supplies
//! exclusive, aligned, little-endian 32-bit MMIO access, clocks and reset, and
//! must check the integration's optional capabilities before accessing them.

#![no_std]
#![deny(missing_docs)]

mod access;
mod registers;
mod values;

pub use access::{DataCommandRegister, ReadClear, ReadWriteCommand};
pub use registers::DwApbI2c;
pub use values::{
    ComponentParameters, Control, DataCommand, DmaControl, Enable, EnableStatus, InterruptBits,
    ReceiveData, SdaHold, Speed, Status, TargetAddress, TransmitAbortSource,
};

#[cfg(test)]
mod tests;
