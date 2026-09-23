//! Typed I²C register values. Setters preserve unrelated bits.

/// Protocol speed encoding; support and actual timing are integration-specific.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Speed {
    /// Standard mode (encoding 1).
    Standard = 1,
    /// Fast or fast-plus mode (encoding 2).
    Fast = 2,
    /// High-speed mode (encoding 3), if implemented.
    High = 3,
}

impl Speed {
    const fn from_encoding(bits: u32) -> Option<Self> {
        match bits {
            1 => Some(Self::Standard),
            2 => Some(Self::Fast),
            3 => Some(Self::High),
            _ => None,
        }
    }
}

/// Controller configuration; update while the controller is disabled.
#[doc(alias = "IC_CON")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Control(u32);

impl Control {
    /// Construct a value from raw bits without validating controller capabilities.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw register bits.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Decode the speed; returns `None` for the reserved zero encoding.
    #[inline]
    pub const fn speed(self) -> Option<Speed> {
        Speed::from_encoding((self.0 >> 1) & 3)
    }

    /// Set the speed encoding; this does not program SCL timing counts.
    #[inline]
    pub const fn with_speed(self, speed: Speed) -> Self {
        Self((self.0 & !0x6) | ((speed as u32) << 1))
    }

    /// Whether master mode is enabled.
    #[inline]
    pub const fn master_mode(self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Set bit 0 while preserving unrelated bits.
    #[inline]
    pub const fn with_master_mode(self, value: bool) -> Self {
        Self((self.0 & !1) | value as u32)
    }

    /// Whether the slave uses ten-bit addresses.
    #[inline]
    pub const fn slave_ten_bit_addressing(self) -> bool {
        self.0 & (1 << 3) != 0
    }

    /// Set bit 3 while preserving unrelated bits.
    #[inline]
    pub const fn with_slave_ten_bit_addressing(self, value: bool) -> Self {
        Self((self.0 & !(1 << 3)) | ((value as u32) << 3))
    }

    /// Whether the master uses ten-bit addresses (some integrations use IC_TAR instead).
    #[inline]
    pub const fn master_ten_bit_addressing(self) -> bool {
        self.0 & (1 << 4) != 0
    }

    /// Set bit 4 while preserving unrelated bits.
    #[inline]
    pub const fn with_master_ten_bit_addressing(self, value: bool) -> Self {
        Self((self.0 & !(1 << 4)) | ((value as u32) << 4))
    }

    /// Whether repeated START is enabled.
    #[inline]
    pub const fn restart_enabled(self) -> bool {
        self.0 & (1 << 5) != 0
    }

    /// Set bit 5 while preserving unrelated bits.
    #[inline]
    pub const fn with_restart_enabled(self, value: bool) -> Self {
        Self((self.0 & !(1 << 5)) | ((value as u32) << 5))
    }

    /// Whether slave mode is disabled.
    #[inline]
    pub const fn slave_disabled(self) -> bool {
        self.0 & (1 << 6) != 0
    }

    /// Set bit 6 while preserving unrelated bits.
    #[inline]
    pub const fn with_slave_disabled(self, value: bool) -> Self {
        Self((self.0 & !(1 << 6)) | ((value as u32) << 6))
    }

    /// Whether slave STOP detection requires an address match.
    #[inline]
    pub const fn stop_detect_if_addressed(self) -> bool {
        self.0 & (1 << 7) != 0
    }

    /// Set bit 7 while preserving unrelated bits.
    #[inline]
    pub const fn with_stop_detect_if_addressed(self, value: bool) -> Self {
        Self((self.0 & !(1 << 7)) | ((value as u32) << 7))
    }

    /// Whether TX_EMPTY also waits for the shift register.
    #[inline]
    pub const fn transmit_empty_control(self) -> bool {
        self.0 & (1 << 8) != 0
    }

    /// Set bit 8 while preserving unrelated bits.
    #[inline]
    pub const fn with_transmit_empty_control(self, value: bool) -> Self {
        Self((self.0 & !(1 << 8)) | ((value as u32) << 8))
    }

    /// Whether a full receive FIFO stretches SCL instead of overflowing.
    #[inline]
    pub const fn receive_fifo_full_hold(self) -> bool {
        self.0 & (1 << 9) != 0
    }

    /// Set bit 9 while preserving unrelated bits.
    #[inline]
    pub const fn with_receive_fifo_full_hold(self, value: bool) -> Self {
        Self((self.0 & !(1 << 9)) | ((value as u32) << 9))
    }

    /// Whether STOP detection requires an active master; may be read-only.
    #[inline]
    pub const fn stop_detect_if_master_active(self) -> bool {
        self.0 & (1 << 10) != 0
    }

    /// Set bit 10 while preserving unrelated bits.
    #[inline]
    pub const fn with_stop_detect_if_master_active(self, value: bool) -> Self {
        Self((self.0 & !(1 << 10)) | ((value as u32) << 10))
    }

    /// Whether optional bus-clear control is enabled.
    #[inline]
    pub const fn bus_clear_control(self) -> bool {
        self.0 & (1 << 11) != 0
    }

    /// Set bit 11 while preserving unrelated bits.
    #[inline]
    pub const fn with_bus_clear_control(self, value: bool) -> Self {
        Self((self.0 & !(1 << 11)) | ((value as u32) << 11))
    }
}

/// Master address and general-call/START-byte selection.
#[doc(alias = "IC_TAR")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct TargetAddress(u32);

impl TargetAddress {
    /// Construct a value from raw bits without validating controller capabilities.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw register bits.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Return the ten-bit address field (seven-bit mode uses its low seven bits).
    #[inline]
    pub const fn address(self) -> u16 {
        (self.0 & 0x3ff) as u16
    }

    /// Replace the address field; panics if `address > 0x3ff`.
    ///
    /// The caller must separately select the correct seven/ten-bit mode.
    #[inline]
    pub const fn with_address(self, address: u16) -> Self {
        assert!(address <= 0x3ff, "address is outside the register");
        Self((self.0 & !0x3ff) | address as u32)
    }

    /// Whether a special command is START byte (one) or general call (zero).
    #[inline]
    pub const fn start_byte(self) -> bool {
        self.0 & (1 << 10) != 0
    }

    /// Set bit 10 while preserving unrelated bits.
    #[inline]
    pub const fn with_start_byte(self, value: bool) -> Self {
        Self((self.0 & !(1 << 10)) | ((value as u32) << 10))
    }

    /// Whether special-command addressing is selected.
    #[inline]
    pub const fn special(self) -> bool {
        self.0 & (1 << 11) != 0
    }

    /// Set bit 11 while preserving unrelated bits.
    #[inline]
    pub const fn with_special(self, value: bool) -> Self {
        Self((self.0 & !(1 << 11)) | ((value as u32) << 11))
    }

    /// Optional dynamic master ten-bit mode; otherwise use IC_CON bit 4.
    #[inline]
    pub const fn ten_bit_addressing(self) -> bool {
        self.0 & (1 << 12) != 0
    }

    /// Set bit 12 while preserving unrelated bits.
    #[inline]
    pub const fn with_ten_bit_addressing(self, value: bool) -> Self {
        Self((self.0 & !(1 << 12)) | ((value as u32) << 12))
    }
}

/// One transmit FIFO command; bit 8 selects a master read request.
#[doc(alias = "IC_DATA_CMD")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct DataCommand(u32);

impl DataCommand {
    /// Construct a value from raw bits without validating controller capabilities.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw register bits.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Construct a plain byte-transmit command, with STOP and RESTART clear.
    #[inline]
    pub const fn write_byte(byte: u8) -> Self {
        Self(byte as u32)
    }

    /// Construct a one-byte master read request, with STOP and RESTART clear.
    #[inline]
    pub const fn read_request() -> Self {
        Self(1 << 8)
    }

    /// Return the outgoing byte field, ignored by a master read request.
    #[inline]
    pub const fn data(self) -> u8 {
        self.0 as u8
    }

    /// Whether this command requests a master read.
    #[inline]
    pub const fn read(self) -> bool {
        self.0 & (1 << 8) != 0
    }

    /// Whether to generate STOP after this command; requires empty-FIFO hold support.
    #[inline]
    pub const fn stop(self) -> bool {
        self.0 & (1 << 9) != 0
    }

    /// Set bit 9 while preserving unrelated bits.
    #[inline]
    pub const fn with_stop(self, value: bool) -> Self {
        Self((self.0 & !(1 << 9)) | ((value as u32) << 9))
    }

    /// Whether to generate RESTART before this command; requires IC_RESTART_EN.
    #[inline]
    pub const fn restart(self) -> bool {
        self.0 & (1 << 10) != 0
    }

    /// Set bit 10 while preserving unrelated bits.
    #[inline]
    pub const fn with_restart(self, value: bool) -> Self {
        Self((self.0 & !(1 << 10)) | ((value as u32) << 10))
    }
}

/// A received byte and its optional slave first-byte marker.
#[doc(alias = "IC_DATA_CMD")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct ReceiveData(u32);

impl ReceiveData {
    /// Construct a value from raw bits without validating controller capabilities.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw register bits.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Return the received byte.
    #[inline]
    pub const fn data(self) -> u8 {
        self.0 as u8
    }

    /// Whether this is the first received byte, if the marker is implemented.
    #[inline]
    pub const fn first_data_byte(self) -> bool {
        self.0 & (1 << 11) != 0
    }
}

/// Interrupt bits shared by status, raw status and mask registers.
#[doc(alias = "IC_INTR_STAT")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct InterruptBits(u32);

impl InterruptBits {
    /// Construct a value from raw bits without validating controller capabilities.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw register bits.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Receive underflow event (bit 0); availability depends on integration.
    pub const RECEIVE_UNDERFLOW: Self = Self(1 << 0);

    /// Receive overflow event (bit 1); availability depends on integration.
    pub const RECEIVE_OVERFLOW: Self = Self(1 << 1);

    /// Receive full event (bit 2); availability depends on integration.
    pub const RECEIVE_FULL: Self = Self(1 << 2);

    /// Transmit overflow event (bit 3); availability depends on integration.
    pub const TRANSMIT_OVERFLOW: Self = Self(1 << 3);

    /// Transmit empty event (bit 4); availability depends on integration.
    pub const TRANSMIT_EMPTY: Self = Self(1 << 4);

    /// Read request event (bit 5); availability depends on integration.
    pub const READ_REQUEST: Self = Self(1 << 5);

    /// Transmit abort event (bit 6); availability depends on integration.
    pub const TRANSMIT_ABORT: Self = Self(1 << 6);

    /// Receive done event (bit 7); availability depends on integration.
    pub const RECEIVE_DONE: Self = Self(1 << 7);

    /// Activity event (bit 8); availability depends on integration.
    pub const ACTIVITY: Self = Self(1 << 8);

    /// Stop detect event (bit 9); availability depends on integration.
    pub const STOP_DETECT: Self = Self(1 << 9);

    /// Start detect event (bit 10); availability depends on integration.
    pub const START_DETECT: Self = Self(1 << 10);

    /// General call event (bit 11); availability depends on integration.
    pub const GENERAL_CALL: Self = Self(1 << 11);

    /// Restart detect event (bit 12); availability depends on integration.
    pub const RESTART_DETECT: Self = Self(1 << 12);

    /// Master on hold event (bit 13); availability depends on integration.
    pub const MASTER_ON_HOLD: Self = Self(1 << 13);

    /// Test whether all selected interrupt bits are set.
    #[inline]
    pub const fn contains(self, events: Self) -> bool {
        self.0 & events.0 == events.0
    }

    /// Set or clear selected bits; in IC_INTR_MASK, one means unmasked.
    #[inline]
    pub const fn with_events(self, events: Self, enabled: bool) -> Self {
        Self((self.0 & !events.0) | if enabled { events.0 } else { 0 })
    }
}

/// Enable and self-clearing abort commands; do not replay via read-modify-write.
#[doc(alias = "IC_ENABLE")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Enable(u32);

impl Enable {
    /// Construct a value from raw bits without validating controller capabilities.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw register bits.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Whether controller enable is requested.
    #[inline]
    pub const fn enabled(self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Set bit 0 while preserving unrelated bits.
    #[inline]
    pub const fn with_enabled(self, value: bool) -> Self {
        Self((self.0 & !1) | value as u32)
    }

    /// Whether a master abort is requested; self-clears when completed.
    #[inline]
    pub const fn abort(self) -> bool {
        self.0 & (1 << 1) != 0
    }

    /// Set bit 1 while preserving unrelated bits.
    #[inline]
    pub const fn with_abort(self, value: bool) -> Self {
        Self((self.0 & !(1 << 1)) | ((value as u32) << 1))
    }

    /// Whether the optional transmit-command block is set.
    #[inline]
    pub const fn transmit_command_block(self) -> bool {
        self.0 & (1 << 2) != 0
    }

    /// Set bit 2 while preserving unrelated bits.
    #[inline]
    pub const fn with_transmit_command_block(self, value: bool) -> Self {
        Self((self.0 & !(1 << 2)) | ((value as u32) << 2))
    }
}

/// Controller activity and FIFO state.
#[doc(alias = "IC_STATUS")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Status(u32);

impl Status {
    /// Construct a value from raw bits without validating controller capabilities.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw register bits.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Whether the controller is active.
    #[inline]
    pub const fn active(self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Whether the transmit FIFO has room.
    #[inline]
    pub const fn transmit_not_full(self) -> bool {
        self.0 & (1 << 1) != 0
    }

    /// Whether the transmit FIFO is empty.
    #[inline]
    pub const fn transmit_empty(self) -> bool {
        self.0 & (1 << 2) != 0
    }

    /// Whether the receive FIFO contains data.
    #[inline]
    pub const fn receive_not_empty(self) -> bool {
        self.0 & (1 << 3) != 0
    }

    /// Whether the receive FIFO is full.
    #[inline]
    pub const fn receive_full(self) -> bool {
        self.0 & (1 << 4) != 0
    }

    /// Whether the master state machine is active.
    #[inline]
    pub const fn master_active(self) -> bool {
        self.0 & (1 << 5) != 0
    }

    /// Whether the slave state machine is active.
    #[inline]
    pub const fn slave_active(self) -> bool {
        self.0 & (1 << 6) != 0
    }
}

/// Actual controller state and slave-disable diagnostics.
#[doc(alias = "IC_ENABLE_STATUS")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct EnableStatus(u32);

impl EnableStatus {
    /// Construct a value from raw bits without validating controller capabilities.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw register bits.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Whether the controller has completed enabling.
    #[inline]
    pub const fn enabled(self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Whether the slave was disabled during a transfer.
    #[inline]
    pub const fn slave_disabled_while_busy(self) -> bool {
        self.0 & (1 << 1) != 0
    }

    /// Whether slave receive data was lost on disable.
    #[inline]
    pub const fn slave_receive_data_lost(self) -> bool {
        self.0 & (1 << 2) != 0
    }
}

/// Optional DMA request enables.
#[doc(alias = "IC_DMA_CR")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct DmaControl(u32);

impl DmaControl {
    /// Construct a value from raw bits without validating controller capabilities.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw register bits.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Whether receive DMA requests are enabled.
    #[inline]
    pub const fn receive_enabled(self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Set bit 0 while preserving unrelated bits.
    #[inline]
    pub const fn with_receive_enabled(self, value: bool) -> Self {
        Self((self.0 & !1) | value as u32)
    }

    /// Whether transmit DMA requests are enabled.
    #[inline]
    pub const fn transmit_enabled(self) -> bool {
        self.0 & (1 << 1) != 0
    }

    /// Set bit 1 while preserving unrelated bits.
    #[inline]
    pub const fn with_transmit_enabled(self, value: bool) -> Self {
        Self((self.0 & !(1 << 1)) | ((value as u32) << 1))
    }
}

/// SDA transmit and optional receive hold counts in input-clock cycles.
#[doc(alias = "IC_SDA_HOLD")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct SdaHold(u32);

impl SdaHold {
    /// Construct a value from raw bits without validating controller capabilities.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw register bits.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Return the transmit hold count.
    #[inline]
    pub const fn transmit(self) -> u16 {
        self.0 as u16
    }

    /// Set the transmit count while preserving receive and reserved bits.
    #[inline]
    pub const fn with_transmit(self, count: u16) -> Self {
        Self((self.0 & !0xffff) | count as u32)
    }

    /// Return the optional receive hold count.
    #[inline]
    pub const fn receive(self) -> u8 {
        (self.0 >> 16) as u8
    }

    /// Set the receive count; check that the component version implements it.
    #[inline]
    pub const fn with_receive(self, count: u8) -> Self {
        Self((self.0 & !0xff0000) | ((count as u32) << 16))
    }
}

/// Transmit-abort diagnostics, captured before reading IC_CLR_TX_ABRT.
#[doc(alias = "IC_TX_ABRT_SOURCE")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct TransmitAbortSource(u32);

impl TransmitAbortSource {
    /// Construct a value from raw bits without validating controller capabilities.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw register bits.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Whether the seven-bit address was not acknowledged.
    #[inline]
    pub const fn seven_bit_address_no_ack(self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Whether the first ten-bit address byte was not acknowledged.
    #[inline]
    pub const fn ten_bit_first_address_no_ack(self) -> bool {
        self.0 & (1 << 1) != 0
    }

    /// Whether the second ten-bit address byte was not acknowledged.
    #[inline]
    pub const fn ten_bit_second_address_no_ack(self) -> bool {
        self.0 & (1 << 2) != 0
    }

    /// Whether transmitted data was not acknowledged.
    #[inline]
    pub const fn data_no_ack(self) -> bool {
        self.0 & (1 << 3) != 0
    }

    /// Whether general call was not acknowledged.
    #[inline]
    pub const fn general_call_no_ack(self) -> bool {
        self.0 & (1 << 4) != 0
    }

    /// Whether master arbitration was lost.
    #[inline]
    pub const fn arbitration_lost(self) -> bool {
        self.0 & (1 << 12) != 0
    }

    /// Whether slave arbitration was lost.
    #[inline]
    pub const fn slave_arbitration_lost(self) -> bool {
        self.0 & (1 << 14) != 0
    }

    /// Whether software requested the abort.
    #[inline]
    pub const fn user_abort(self) -> bool {
        self.0 & (1 << 16) != 0
    }

    /// Number of flushed transmit FIFO entries (bits 31:23), if implemented.
    #[inline]
    pub const fn flushed_entries(self) -> u16 {
        (self.0 >> 23) as u16
    }
}

/// Optional encoded synthesis parameters. None means parameters are unavailable.
#[doc(alias = "IC_COMP_PARAM_1")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct ComponentParameters(u32);

impl ComponentParameters {
    /// Construct a value from raw bits without validating controller capabilities.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw register bits.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Whether encoded synthesis parameters are present.
    #[inline]
    pub const fn encoded_parameters(self) -> bool {
        self.0 & (1 << 7) != 0
    }

    /// Transmit FIFO depth, decoding the hardware's depth-minus-one field.
    #[inline]
    pub const fn transmit_fifo_depth(self) -> Option<u16> {
        if self.encoded_parameters() {
            Some(((self.0 >> 16) & 0xff) as u16 + 1)
        } else {
            None
        }
    }

    /// Receive FIFO depth, decoding the hardware's depth-minus-one field.
    #[inline]
    pub const fn receive_fifo_depth(self) -> Option<u16> {
        if self.encoded_parameters() {
            Some(((self.0 >> 8) & 0xff) as u16 + 1)
        } else {
            None
        }
    }

    /// Whether DMA is synthesized; unavailable when parameters are not encoded.
    #[inline]
    pub const fn has_dma(self) -> Option<bool> {
        if self.encoded_parameters() {
            Some(self.0 & (1 << 6) != 0)
        } else {
            None
        }
    }

    /// Maximum supported protocol speed; `None` also covers reserved encoding zero.
    #[inline]
    pub const fn maximum_speed(self) -> Option<Speed> {
        if self.encoded_parameters() {
            Speed::from_encoding((self.0 >> 2) & 3)
        } else {
            None
        }
    }

    /// APB data width in bits; `None` also covers reserved encoding three.
    ///
    /// This metadata does not adapt this crate's 32-bit MMIO accesses.
    #[inline]
    pub const fn apb_data_width(self) -> Option<u8> {
        if !self.encoded_parameters() {
            return None;
        }
        match self.0 & 3 {
            0 => Some(8),
            1 => Some(16),
            2 => Some(32),
            _ => None,
        }
    }
}
