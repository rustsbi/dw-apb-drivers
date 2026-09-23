//! Synopsys DesignWare APB I²C registers.

use volatile_register::{RO, RW};

use crate::{
    ComponentParameters, Control, DataCommandRegister, DmaControl, Enable, EnableStatus,
    InterruptBits, ReadClear, ReadWriteCommand, SdaHold, Status, TargetAddress,
    TransmitAbortSource,
};

/// DW_apb_i2c common register layout with little-endian 32-bit accesses.
///
/// Access only implemented registers. Timing, FIFO depth, slave mode, DMA and
/// high-speed support depend on synthesis and SoC integration. The reserved gap
/// after `0xa8` includes newer bus-clear/SMBus extensions not modeled here.
///
/// References: Linux `i2c-designware-core.h`, U-Boot `designware_i2c.h` and the
/// RP2040 register reference. See the crate README for source links.
#[doc(alias = "DW_apb_i2c")]
#[repr(C)]
pub struct DwApbI2c {
    /// `0x00 IC_CON`: Controller mode and protocol configuration; change while disabled.
    #[doc(alias = "IC_CON")]
    pub control: RW<Control>,
    /// `0x04 IC_TAR`: Master target address and special-command selection.
    #[doc(alias = "IC_TAR")]
    pub target_address: RW<TargetAddress>,
    /// `0x08 IC_SAR`: Local slave address in bits 9:0.
    #[doc(alias = "IC_SAR")]
    pub slave_address: RW<u32>,
    /// `0x0c IC_HS_MADDR`: optional high-speed master code in bits 2:0.
    #[doc(alias = "IC_HS_MADDR")]
    pub high_speed_master_code: RW<u32>,
    /// `0x10 IC_DATA_CMD`: Receive FIFO pop or transmit/read-command FIFO push.
    #[doc(alias = "IC_DATA_CMD")]
    pub data_command: DataCommandRegister,
    /// `0x14 IC_SS_SCL_HCNT`: Standard-mode SCL high count; timing depends on the input clock.
    #[doc(alias = "IC_SS_SCL_HCNT")]
    pub standard_scl_high: RW<u32>,
    /// `0x18 IC_SS_SCL_LCNT`: Standard-mode SCL low count.
    #[doc(alias = "IC_SS_SCL_LCNT")]
    pub standard_scl_low: RW<u32>,
    /// `0x1c IC_FS_SCL_HCNT`: Fast/fast-plus SCL high count.
    #[doc(alias = "IC_FS_SCL_HCNT")]
    pub fast_scl_high: RW<u32>,
    /// `0x20 IC_FS_SCL_LCNT`: Fast/fast-plus SCL low count.
    #[doc(alias = "IC_FS_SCL_LCNT")]
    pub fast_scl_low: RW<u32>,
    /// `0x24 IC_HS_SCL_HCNT`: Optional high-speed SCL high count.
    #[doc(alias = "IC_HS_SCL_HCNT")]
    pub high_scl_high: RW<u32>,
    /// `0x28 IC_HS_SCL_LCNT`: Optional high-speed SCL low count.
    #[doc(alias = "IC_HS_SCL_LCNT")]
    pub high_scl_low: RW<u32>,
    /// `0x2c IC_INTR_STAT`: Interrupt status after masking.
    #[doc(alias = "IC_INTR_STAT")]
    pub interrupt_status: RO<InterruptBits>,
    /// `0x30 IC_INTR_MASK`: One unmasks the corresponding interrupt; zero masks it.
    #[doc(alias = "IC_INTR_MASK")]
    pub interrupt_mask: RW<InterruptBits>,
    /// `0x34 IC_RAW_INTR_STAT`: Interrupt status before masking.
    #[doc(alias = "IC_RAW_INTR_STAT")]
    pub raw_interrupt_status: RO<InterruptBits>,
    /// `0x38 IC_RX_TL`: RX_FULL asserts above this FIFO level; respect the integrated depth.
    #[doc(alias = "IC_RX_TL")]
    pub receive_threshold: RW<u32>,
    /// `0x3c IC_TX_TL`: TX_EMPTY asserts at or below this FIFO level.
    #[doc(alias = "IC_TX_TL")]
    pub transmit_threshold: RW<u32>,
    /// `0x40 IC_CLR_INTR`: Read to clear interrupts.
    #[doc(alias = "IC_CLR_INTR")]
    pub clear_interrupts: ReadClear,
    /// `0x44 IC_CLR_RX_UNDER`: Read to clear receive underflow.
    #[doc(alias = "IC_CLR_RX_UNDER")]
    pub clear_receive_underflow: ReadClear,
    /// `0x48 IC_CLR_RX_OVER`: Read to clear receive overflow.
    #[doc(alias = "IC_CLR_RX_OVER")]
    pub clear_receive_overflow: ReadClear,
    /// `0x4c IC_CLR_TX_OVER`: Read to clear transmit overflow.
    #[doc(alias = "IC_CLR_TX_OVER")]
    pub clear_transmit_overflow: ReadClear,
    /// `0x50 IC_CLR_RD_REQ`: Read to clear read request.
    #[doc(alias = "IC_CLR_RD_REQ")]
    pub clear_read_request: ReadClear,
    /// `0x54 IC_CLR_TX_ABRT`: Read to clear transmit abort.
    #[doc(alias = "IC_CLR_TX_ABRT")]
    pub clear_transmit_abort: ReadClear,
    /// `0x58 IC_CLR_RX_DONE`: Read to clear receive done.
    #[doc(alias = "IC_CLR_RX_DONE")]
    pub clear_receive_done: ReadClear,
    /// `0x5c IC_CLR_ACTIVITY`: Read to clear activity.
    #[doc(alias = "IC_CLR_ACTIVITY")]
    pub clear_activity: ReadClear,
    /// `0x60 IC_CLR_STOP_DET`: Read to clear stop detect.
    #[doc(alias = "IC_CLR_STOP_DET")]
    pub clear_stop_detect: ReadClear,
    /// `0x64 IC_CLR_START_DET`: Read to clear start detect.
    #[doc(alias = "IC_CLR_START_DET")]
    pub clear_start_detect: ReadClear,
    /// `0x68 IC_CLR_GEN_CALL`: Read to clear general call.
    #[doc(alias = "IC_CLR_GEN_CALL")]
    pub clear_general_call: ReadClear,
    /// `0x6c IC_ENABLE`: Enable, abort and optional transmit-command block; no read-modify-write.
    #[doc(alias = "IC_ENABLE")]
    pub enable: ReadWriteCommand<Enable>,
    /// `0x70 IC_STATUS`: Activity and FIFO state.
    #[doc(alias = "IC_STATUS")]
    pub status: RO<Status>,
    /// `0x74 IC_TXFLR`: Current transmit FIFO entry count.
    #[doc(alias = "IC_TXFLR")]
    pub transmit_fifo_level: RO<u32>,
    /// `0x78 IC_RXFLR`: Current receive FIFO entry count.
    #[doc(alias = "IC_RXFLR")]
    pub receive_fifo_level: RO<u32>,
    /// `0x7c IC_SDA_HOLD`: SDA hold counts; availability and receive field depend on version.
    #[doc(alias = "IC_SDA_HOLD")]
    pub sda_hold: RW<SdaHold>,
    /// `0x80 IC_TX_ABRT_SOURCE`: Abort causes and flushed-byte count; capture before clearing TX_ABRT.
    #[doc(alias = "IC_TX_ABRT_SOURCE")]
    pub transmit_abort_source: RO<TransmitAbortSource>,
    /// `0x84 IC_SLV_DATA_NACK_ONLY`: Optional slave receive NACK control in bit zero.
    #[doc(alias = "IC_SLV_DATA_NACK_ONLY")]
    pub slave_data_nack_only: RW<u32>,
    /// `0x88 IC_DMA_CR`: Optional receive/transmit DMA request enables.
    #[doc(alias = "IC_DMA_CR")]
    pub dma_control: RW<DmaControl>,
    /// `0x8c IC_DMA_TDLR`: Optional transmit DMA threshold.
    #[doc(alias = "IC_DMA_TDLR")]
    pub dma_transmit_level: RW<u32>,
    /// `0x90 IC_DMA_RDLR`: Optional receive DMA threshold, encoded as entry count minus one.
    #[doc(alias = "IC_DMA_RDLR")]
    pub dma_receive_level: RW<u32>,
    /// `0x94 IC_SDA_SETUP`: Slave SDA setup count; obey the integration timing constraints.
    #[doc(alias = "IC_SDA_SETUP")]
    pub sda_setup: RW<u32>,
    /// `0x98 IC_ACK_GENERAL_CALL`: One in bit zero enables acknowledgement of general calls.
    #[doc(alias = "IC_ACK_GENERAL_CALL")]
    pub acknowledge_general_call: RW<u32>,
    /// `0x9c IC_ENABLE_STATUS`: Actual enable state and slave-disable status.
    #[doc(alias = "IC_ENABLE_STATUS")]
    pub enable_status: RO<EnableStatus>,
    /// `0xa0 IC_FS_SPKLEN`: Fast/standard spike filter count; optional.
    #[doc(alias = "IC_FS_SPKLEN")]
    pub fast_spike_length: RW<u32>,
    /// `0xa4 IC_HS_SPKLEN`: optional high-speed spike filter count.
    #[doc(alias = "IC_HS_SPKLEN")]
    pub high_spike_length: RW<u32>,
    /// `0xa8 IC_CLR_RESTART_DET`: Read to clear restart detection; optional.
    #[doc(alias = "IC_CLR_RESTART_DET")]
    pub clear_restart_detect: ReadClear,
    _reserved_ac: [u32; 18],
    /// `0xf4 IC_COMP_PARAM_1`: Encoded integration parameters; check ADD_ENCODED_PARAMS.
    #[doc(alias = "IC_COMP_PARAM_1")]
    pub component_parameters: RO<ComponentParameters>,
    /// `0xf8 IC_COMP_VERSION`: Component version encoding.
    #[doc(alias = "IC_COMP_VERSION")]
    pub component_version: RO<u32>,
    /// `0xfc IC_COMP_TYPE`: Component identifier, normally 0x44570140.
    #[doc(alias = "IC_COMP_TYPE")]
    pub component_type: RO<u32>,
}
