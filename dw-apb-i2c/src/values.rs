//! Typed I²C register values. Setters preserve unrelated bits.
//!
//! Register and bit-field aliases use the symbols in the vendor manuals:
//! RP2040 Datasheet §4.3.17 and Intel document 872352, I2C MMIO chapters.
//! The primary IP reference is Synopsys DW_apb_i2c Databook 2.05a; see the README
//! for its authenticated links and the public integration references.

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
    #[doc(alias = "SPEED")]
    #[inline]
    pub const fn speed(self) -> Option<Speed> {
        Speed::from_encoding((self.0 >> 1) & 3)
    }

    /// Set the speed encoding; this does not program SCL timing counts.
    #[doc(alias = "SPEED")]
    #[inline]
    pub const fn with_speed(self, speed: Speed) -> Self {
        Self((self.0 & !0x6) | ((speed as u32) << 1))
    }

    /// Whether master mode is enabled.
    #[doc(alias = "MASTER_MODE")]
    #[inline]
    pub const fn master_mode(self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Set bit 0 while preserving unrelated bits.
    #[doc(alias = "MASTER_MODE")]
    #[inline]
    pub const fn with_master_mode(self, value: bool) -> Self {
        Self((self.0 & !1) | value as u32)
    }

    /// Whether the slave uses ten-bit addresses.
    #[doc(alias = "IC_10BITADDR_SLAVE")]
    #[inline]
    pub const fn slave_ten_bit_addressing(self) -> bool {
        self.0 & (1 << 3) != 0
    }

    /// Set bit 3 while preserving unrelated bits.
    #[doc(alias = "IC_10BITADDR_SLAVE")]
    #[inline]
    pub const fn with_slave_ten_bit_addressing(self, value: bool) -> Self {
        Self((self.0 & !(1 << 3)) | ((value as u32) << 3))
    }

    /// Whether the master uses ten-bit addresses (some integrations use IC_TAR instead).
    #[doc(alias = "IC_10BITADDR_MASTER")]
    #[inline]
    pub const fn master_ten_bit_addressing(self) -> bool {
        self.0 & (1 << 4) != 0
    }

    /// Set bit 4 while preserving unrelated bits.
    #[doc(alias = "IC_10BITADDR_MASTER")]
    #[inline]
    pub const fn with_master_ten_bit_addressing(self, value: bool) -> Self {
        Self((self.0 & !(1 << 4)) | ((value as u32) << 4))
    }

    /// Whether repeated START is enabled.
    #[doc(alias = "IC_RESTART_EN")]
    #[inline]
    pub const fn restart_enabled(self) -> bool {
        self.0 & (1 << 5) != 0
    }

    /// Set bit 5 while preserving unrelated bits.
    #[doc(alias = "IC_RESTART_EN")]
    #[inline]
    pub const fn with_restart_enabled(self, value: bool) -> Self {
        Self((self.0 & !(1 << 5)) | ((value as u32) << 5))
    }

    /// Whether slave mode is disabled.
    #[doc(alias = "IC_SLAVE_DISABLE")]
    #[inline]
    pub const fn slave_disabled(self) -> bool {
        self.0 & (1 << 6) != 0
    }

    /// Set bit 6 while preserving unrelated bits.
    #[doc(alias = "IC_SLAVE_DISABLE")]
    #[inline]
    pub const fn with_slave_disabled(self, value: bool) -> Self {
        Self((self.0 & !(1 << 6)) | ((value as u32) << 6))
    }

    /// Whether slave STOP detection requires an address match.
    #[doc(alias = "STOP_DET_IFADDRESSED")]
    #[inline]
    pub const fn stop_detect_if_addressed(self) -> bool {
        self.0 & (1 << 7) != 0
    }

    /// Set bit 7 while preserving unrelated bits.
    #[doc(alias = "STOP_DET_IFADDRESSED")]
    #[inline]
    pub const fn with_stop_detect_if_addressed(self, value: bool) -> Self {
        Self((self.0 & !(1 << 7)) | ((value as u32) << 7))
    }

    /// Whether TX_EMPTY also waits for the shift register.
    #[doc(alias = "TX_EMPTY_CTRL")]
    #[inline]
    pub const fn transmit_empty_control(self) -> bool {
        self.0 & (1 << 8) != 0
    }

    /// Set bit 8 while preserving unrelated bits.
    #[doc(alias = "TX_EMPTY_CTRL")]
    #[inline]
    pub const fn with_transmit_empty_control(self, value: bool) -> Self {
        Self((self.0 & !(1 << 8)) | ((value as u32) << 8))
    }

    /// Whether a full receive FIFO stretches SCL instead of overflowing.
    #[doc(alias = "RX_FIFO_FULL_HLD_CTRL")]
    #[inline]
    pub const fn receive_fifo_full_hold(self) -> bool {
        self.0 & (1 << 9) != 0
    }

    /// Set bit 9 while preserving unrelated bits.
    #[doc(alias = "RX_FIFO_FULL_HLD_CTRL")]
    #[inline]
    pub const fn with_receive_fifo_full_hold(self, value: bool) -> Self {
        Self((self.0 & !(1 << 9)) | ((value as u32) << 9))
    }

    /// Whether STOP detection requires an active master; may be read-only.
    #[doc(alias = "STOP_DET_IF_MASTER_ACTIVE")]
    #[inline]
    pub const fn stop_detect_if_master_active(self) -> bool {
        self.0 & (1 << 10) != 0
    }

    /// Set bit 10 while preserving unrelated bits.
    #[doc(alias = "STOP_DET_IF_MASTER_ACTIVE")]
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
    #[doc(alias = "IC_TAR")]
    #[inline]
    pub const fn address(self) -> u16 {
        (self.0 & 0x3ff) as u16
    }

    /// Replace the address field; panics if `address > 0x3ff`.
    ///
    /// The caller must separately select the correct seven/ten-bit mode.
    #[doc(alias = "IC_TAR")]
    #[inline]
    pub const fn with_address(self, address: u16) -> Self {
        assert!(address <= 0x3ff, "address is outside the register");
        Self((self.0 & !0x3ff) | address as u32)
    }

    /// Whether a special command is START byte (one) or general call (zero).
    #[doc(alias = "GC_OR_START")]
    #[inline]
    pub const fn start_byte(self) -> bool {
        self.0 & (1 << 10) != 0
    }

    /// Set bit 10 while preserving unrelated bits.
    #[doc(alias = "GC_OR_START")]
    #[inline]
    pub const fn with_start_byte(self, value: bool) -> Self {
        Self((self.0 & !(1 << 10)) | ((value as u32) << 10))
    }

    /// Whether special-command addressing is selected.
    #[doc(alias = "SPECIAL")]
    #[inline]
    pub const fn special(self) -> bool {
        self.0 & (1 << 11) != 0
    }

    /// Set bit 11 while preserving unrelated bits.
    #[doc(alias = "SPECIAL")]
    #[inline]
    pub const fn with_special(self, value: bool) -> Self {
        Self((self.0 & !(1 << 11)) | ((value as u32) << 11))
    }

    /// Optional dynamic master ten-bit mode; otherwise use IC_CON bit 4.
    #[doc(alias = "IC_10BITADDR_MASTER")]
    #[inline]
    pub const fn ten_bit_addressing(self) -> bool {
        self.0 & (1 << 12) != 0
    }

    /// Set bit 12 while preserving unrelated bits.
    #[doc(alias = "IC_10BITADDR_MASTER")]
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
    #[doc(alias = "DAT")]
    #[inline]
    pub const fn write_byte(byte: u8) -> Self {
        Self(byte as u32)
    }

    /// Construct a one-byte master read request, with STOP and RESTART clear.
    #[doc(alias = "CMD")]
    #[inline]
    pub const fn read_request() -> Self {
        Self(1 << 8)
    }

    /// Return the outgoing byte field, ignored by a master read request.
    #[doc(alias = "DAT")]
    #[inline]
    pub const fn data(self) -> u8 {
        self.0 as u8
    }

    /// Whether this command requests a master read.
    #[doc(alias = "CMD")]
    #[inline]
    pub const fn read(self) -> bool {
        self.0 & (1 << 8) != 0
    }

    /// Whether to generate STOP after this command; requires empty-FIFO hold support.
    #[doc(alias = "STOP")]
    #[inline]
    pub const fn stop(self) -> bool {
        self.0 & (1 << 9) != 0
    }

    /// Set bit 9 while preserving unrelated bits.
    #[doc(alias = "STOP")]
    #[inline]
    pub const fn with_stop(self, value: bool) -> Self {
        Self((self.0 & !(1 << 9)) | ((value as u32) << 9))
    }

    /// Whether to generate RESTART before this command; requires IC_RESTART_EN.
    #[doc(alias = "RESTART")]
    #[inline]
    pub const fn restart(self) -> bool {
        self.0 & (1 << 10) != 0
    }

    /// Set bit 10 while preserving unrelated bits.
    #[doc(alias = "RESTART")]
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
    #[doc(alias = "DAT")]
    #[inline]
    pub const fn data(self) -> u8 {
        self.0 as u8
    }

    /// Whether this is the first received byte, if the marker is implemented.
    #[doc(alias = "FIRST_DATA_BYTE")]
    #[inline]
    pub const fn first_data_byte(self) -> bool {
        self.0 & (1 << 11) != 0
    }
}

/// Interrupt bits shared by status, raw status and mask registers.
#[doc(alias("IC_INTR_STAT", "IC_RAW_INTR_STAT", "IC_INTR_MASK"))]
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
    #[doc(alias("RX_UNDER", "R_RX_UNDER", "M_RX_UNDER"))]
    pub const RECEIVE_UNDERFLOW: Self = Self(1 << 0);

    /// Receive overflow event (bit 1); availability depends on integration.
    #[doc(alias("RX_OVER", "R_RX_OVER", "M_RX_OVER"))]
    pub const RECEIVE_OVERFLOW: Self = Self(1 << 1);

    /// Receive full event (bit 2); availability depends on integration.
    #[doc(alias("RX_FULL", "R_RX_FULL", "M_RX_FULL"))]
    pub const RECEIVE_FULL: Self = Self(1 << 2);

    /// Transmit overflow event (bit 3); availability depends on integration.
    #[doc(alias("TX_OVER", "R_TX_OVER", "M_TX_OVER"))]
    pub const TRANSMIT_OVERFLOW: Self = Self(1 << 3);

    /// Transmit empty event (bit 4); availability depends on integration.
    #[doc(alias("TX_EMPTY", "R_TX_EMPTY", "M_TX_EMPTY"))]
    pub const TRANSMIT_EMPTY: Self = Self(1 << 4);

    /// Read request event (bit 5); availability depends on integration.
    #[doc(alias("RD_REQ", "R_RD_REQ", "M_RD_REQ"))]
    pub const READ_REQUEST: Self = Self(1 << 5);

    /// Transmit abort event (bit 6); availability depends on integration.
    #[doc(alias("TX_ABRT", "R_TX_ABRT", "M_TX_ABRT"))]
    pub const TRANSMIT_ABORT: Self = Self(1 << 6);

    /// Receive done event (bit 7); availability depends on integration.
    #[doc(alias("RX_DONE", "R_RX_DONE", "M_RX_DONE"))]
    pub const RECEIVE_DONE: Self = Self(1 << 7);

    /// Activity event (bit 8); availability depends on integration.
    #[doc(alias("R_ACTIVITY", "M_ACTIVITY"))]
    pub const ACTIVITY: Self = Self(1 << 8);

    /// Stop detect event (bit 9); availability depends on integration.
    #[doc(alias("STOP_DET", "R_STOP_DET", "M_STOP_DET"))]
    pub const STOP_DETECT: Self = Self(1 << 9);

    /// Start detect event (bit 10); availability depends on integration.
    #[doc(alias("START_DET", "R_START_DET", "M_START_DET"))]
    pub const START_DETECT: Self = Self(1 << 10);

    /// General call event (bit 11); availability depends on integration.
    #[doc(alias("GEN_CALL", "R_GEN_CALL", "M_GEN_CALL"))]
    pub const GENERAL_CALL: Self = Self(1 << 11);

    /// Restart detect event (bit 12); availability depends on integration.
    #[doc(alias("RESTART_DET", "R_RESTART_DET", "M_RESTART_DET"))]
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
    #[doc(alias = "ENABLE")]
    #[inline]
    pub const fn enabled(self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Set bit 0 while preserving unrelated bits.
    #[doc(alias = "ENABLE")]
    #[inline]
    pub const fn with_enabled(self, value: bool) -> Self {
        Self((self.0 & !1) | value as u32)
    }

    /// Whether a master abort is requested; self-clears when completed.
    #[doc(alias = "ABORT")]
    #[inline]
    pub const fn abort(self) -> bool {
        self.0 & (1 << 1) != 0
    }

    /// Set bit 1 while preserving unrelated bits.
    #[doc(alias = "ABORT")]
    #[inline]
    pub const fn with_abort(self, value: bool) -> Self {
        Self((self.0 & !(1 << 1)) | ((value as u32) << 1))
    }

    /// Whether the optional transmit-command block is set.
    #[doc(alias = "TX_CMD_BLOCK")]
    #[inline]
    pub const fn transmit_command_block(self) -> bool {
        self.0 & (1 << 2) != 0
    }

    /// Set bit 2 while preserving unrelated bits.
    #[doc(alias = "TX_CMD_BLOCK")]
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
    #[doc(alias = "ACTIVITY")]
    #[inline]
    pub const fn active(self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Whether the transmit FIFO has room.
    #[doc(alias = "TFNF")]
    #[inline]
    pub const fn transmit_not_full(self) -> bool {
        self.0 & (1 << 1) != 0
    }

    /// Whether the transmit FIFO is empty.
    #[doc(alias = "TFE")]
    #[inline]
    pub const fn transmit_empty(self) -> bool {
        self.0 & (1 << 2) != 0
    }

    /// Whether the receive FIFO contains data.
    #[doc(alias = "RFNE")]
    #[inline]
    pub const fn receive_not_empty(self) -> bool {
        self.0 & (1 << 3) != 0
    }

    /// Whether the receive FIFO is full.
    #[doc(alias = "RFF")]
    #[inline]
    pub const fn receive_full(self) -> bool {
        self.0 & (1 << 4) != 0
    }

    /// Whether the master state machine is active.
    #[doc(alias = "MST_ACTIVITY")]
    #[inline]
    pub const fn master_active(self) -> bool {
        self.0 & (1 << 5) != 0
    }

    /// Whether the slave state machine is active.
    #[doc(alias = "SLV_ACTIVITY")]
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
    #[doc(alias = "IC_EN")]
    #[inline]
    pub const fn enabled(self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Whether the slave was disabled during a transfer.
    #[doc(alias = "SLV_DISABLED_WHILE_BUSY")]
    #[inline]
    pub const fn slave_disabled_while_busy(self) -> bool {
        self.0 & (1 << 1) != 0
    }

    /// Whether slave receive data was lost on disable.
    #[doc(alias = "SLV_RX_DATA_LOST")]
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
    #[doc(alias = "RDMAE")]
    #[inline]
    pub const fn receive_enabled(self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Set bit 0 while preserving unrelated bits.
    #[doc(alias = "RDMAE")]
    #[inline]
    pub const fn with_receive_enabled(self, value: bool) -> Self {
        Self((self.0 & !1) | value as u32)
    }

    /// Whether transmit DMA requests are enabled.
    #[doc(alias = "TDMAE")]
    #[inline]
    pub const fn transmit_enabled(self) -> bool {
        self.0 & (1 << 1) != 0
    }

    /// Set bit 1 while preserving unrelated bits.
    #[doc(alias = "TDMAE")]
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
    #[doc(alias = "IC_SDA_TX_HOLD")]
    #[inline]
    pub const fn transmit(self) -> u16 {
        self.0 as u16
    }

    /// Set the transmit count while preserving receive and reserved bits.
    #[doc(alias = "IC_SDA_TX_HOLD")]
    #[inline]
    pub const fn with_transmit(self, count: u16) -> Self {
        Self((self.0 & !0xffff) | count as u32)
    }

    /// Return the optional receive hold count.
    #[doc(alias = "IC_SDA_RX_HOLD")]
    #[inline]
    pub const fn receive(self) -> u8 {
        (self.0 >> 16) as u8
    }

    /// Set the receive count; check that the component version implements it.
    #[doc(alias = "IC_SDA_RX_HOLD")]
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
    #[doc(alias = "ABRT_7B_ADDR_NOACK")]
    #[inline]
    pub const fn seven_bit_address_no_ack(self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Whether the first ten-bit address byte was not acknowledged.
    #[doc(alias = "ABRT_10ADDR1_NOACK")]
    #[inline]
    pub const fn ten_bit_first_address_no_ack(self) -> bool {
        self.0 & (1 << 1) != 0
    }

    /// Whether the second ten-bit address byte was not acknowledged.
    #[doc(alias = "ABRT_10ADDR2_NOACK")]
    #[inline]
    pub const fn ten_bit_second_address_no_ack(self) -> bool {
        self.0 & (1 << 2) != 0
    }

    /// Whether transmitted data was not acknowledged.
    #[doc(alias = "ABRT_TXDATA_NOACK")]
    #[inline]
    pub const fn data_no_ack(self) -> bool {
        self.0 & (1 << 3) != 0
    }

    /// Whether general call was not acknowledged.
    #[doc(alias = "ABRT_GCALL_NOACK")]
    #[inline]
    pub const fn general_call_no_ack(self) -> bool {
        self.0 & (1 << 4) != 0
    }

    /// Whether master arbitration was lost.
    #[doc(alias = "ARB_LOST")]
    #[inline]
    pub const fn arbitration_lost(self) -> bool {
        self.0 & (1 << 12) != 0
    }

    /// Whether slave arbitration was lost.
    #[doc(alias = "ABRT_SLV_ARBLOST")]
    #[inline]
    pub const fn slave_arbitration_lost(self) -> bool {
        self.0 & (1 << 14) != 0
    }

    /// Whether software requested the abort.
    #[doc(alias = "ABRT_USER_ABRT")]
    #[inline]
    pub const fn user_abort(self) -> bool {
        self.0 & (1 << 16) != 0
    }

    /// Number of flushed transmit FIFO entries (bits 31:23), if implemented.
    #[doc(alias = "TX_FLUSH_CNT")]
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
    #[doc(alias = "ADD_ENCODED_PARAMS")]
    #[inline]
    pub const fn encoded_parameters(self) -> bool {
        self.0 & (1 << 7) != 0
    }

    /// Transmit FIFO depth, decoding the hardware's depth-minus-one field.
    #[doc(alias = "TX_BUFFER_DEPTH")]
    #[inline]
    pub const fn transmit_fifo_depth(self) -> Option<u16> {
        if self.encoded_parameters() {
            Some(((self.0 >> 16) & 0xff) as u16 + 1)
        } else {
            None
        }
    }

    /// Receive FIFO depth, decoding the hardware's depth-minus-one field.
    #[doc(alias = "RX_BUFFER_DEPTH")]
    #[inline]
    pub const fn receive_fifo_depth(self) -> Option<u16> {
        if self.encoded_parameters() {
            Some(((self.0 >> 8) & 0xff) as u16 + 1)
        } else {
            None
        }
    }

    /// Whether DMA is synthesized; unavailable when parameters are not encoded.
    #[doc(alias = "HAS_DMA")]
    #[inline]
    pub const fn has_dma(self) -> Option<bool> {
        if self.encoded_parameters() {
            Some(self.0 & (1 << 6) != 0)
        } else {
            None
        }
    }

    /// Maximum supported protocol speed; `None` also covers reserved encoding zero.
    #[doc(alias = "MAX_SPEED_MODE")]
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
    #[doc(alias = "APB_DATA_WIDTH")]
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
