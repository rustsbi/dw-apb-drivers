//! Access policies for destructive reads and command writes.
//!
//! Reference: RP2040 Datasheet §4.3.16 (table 451) and §4.3.17 entries for
//! `IC_CLR_*`, `IC_ENABLE` and `IC_DATA_CMD`; see the README for the official PDF.

use volatile_register::{RO, RW};

use crate::{DataCommand, ReceiveData};

/// An interrupt acknowledgement register: reading clears the associated event.
///
/// There is no write or modify operation.
///
/// ```compile_fail
/// # fn example(registers: &dw_apb_i2c::DwApbI2c) {
/// unsafe { registers.clear_interrupts.write(1); }
/// # }
/// ```
#[repr(transparent)]
pub struct ReadClear {
    register: RO<u32>,
}

impl ReadClear {
    /// Read the register and acknowledge its event.
    ///
    /// # Safety
    /// Clearing the associated interrupt and its auxiliary state must be valid.
    /// In particular, clearing TX_ABRT also releases the transmit FIFO flush.
    #[inline]
    pub unsafe fn read(&self) -> u32 {
        self.register.read()
    }
}

/// A command/status register with exact writes and no read-modify-write operation.
///
/// ```compile_fail
/// # fn example(registers: &dw_apb_i2c::DwApbI2c) {
/// unsafe { registers.enable.modify(|value| value); }
/// # }
/// ```
#[repr(transparent)]
pub struct ReadWriteCommand<T: Copy> {
    register: RW<T>,
}

impl<T: Copy> ReadWriteCommand<T> {
    /// Read the current command/status bits without changing them.
    #[inline]
    pub fn read(&self) -> T {
        self.register.read()
    }

    /// Write exactly one command value, without reading first.
    ///
    /// # Safety
    /// The command and all its hardware side effects must be valid in the
    /// current controller state. Do not replay self-clearing command bits.
    #[inline]
    pub unsafe fn write(&self, value: T) {
        // SAFETY: The caller establishes the command's validity.
        unsafe { self.register.write(value) };
    }
}

/// `IC_DATA_CMD`: a receive FIFO pop port and a transmit/read-command FIFO push port.
///
/// Read and write values have different meanings. There is no modify operation.
///
/// ```compile_fail
/// # fn example(registers: &dw_apb_i2c::DwApbI2c) {
/// unsafe { registers.data_command.modify(|value| value); }
/// # }
/// ```
#[doc(alias = "IC_DATA_CMD")]
#[repr(transparent)]
pub struct DataCommandRegister {
    register: RW<u32>,
}

impl DataCommandRegister {
    /// Pop one received byte and its optional first-byte marker.
    ///
    /// # Safety
    /// The receive FIFO must contain data, and consuming it must not conflict
    /// with another user, interrupt handler or DMA engine.
    #[inline]
    pub unsafe fn read(&self) -> ReceiveData {
        ReceiveData::from_bits(self.register.read())
    }

    /// Push one transmitted byte or master read request.
    ///
    /// # Safety
    /// The transmit FIFO must have room, the controller must be configured for
    /// this command, and no other user or DMA engine may race this operation.
    #[inline]
    pub unsafe fn write(&self, command: DataCommand) {
        // SAFETY: The caller establishes FIFO space and command validity.
        unsafe { self.register.write(command.bits()) };
    }
}
