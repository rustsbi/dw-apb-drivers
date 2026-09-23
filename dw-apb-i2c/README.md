# DW APB I²C

`no_std` register layer for Synopsys DW_apb_i2c with little-endian 32-bit MMIO
accesses. It follows `dw-apb-gpio`: a `#[repr(C)]` block, documented offsets and
hardware-name aliases, `volatile-register` accessors, and transparent typed
values with `from_bits`, `bits` and setters that preserve unrelated bits.

`DwApbI2c` covers the common register map through `IC_COMP_TYPE` at `0xfc`,
including optional high-speed, slave and DMA registers. The reserved region
`0xac..0xf4` is left unexposed; newer bus-clear and SMBus extensions are outside
this initial implementation. This crate does not implement I²C transactions,
`embedded-hal::i2c::I2c`, clock setup, pin configuration or peripheral ownership.

## Access semantics

- `IC_DATA_CMD` has separate `DataCommand` and `ReceiveData` values. Reads pop
  the receive FIFO; writes push transmit bytes or master read requests. Both
  operations are unsafe and there is no `modify` method.
- `IC_CLR_*` uses `ReadClear`: its unsafe read acknowledges events. It has no
  write or modify method. Capture abort causes before clearing `IC_CLR_TX_ABRT`.
- `IC_ENABLE` uses `ReadWriteCommand<Enable>` so a read-modify-write cannot
  inadvertently replay the self-clearing ABORT command.
- `IC_INTR_MASK` uses **one to unmask**, unlike GPIO's mask register.
- Optional fields/registers must be checked against the SoC integration and
  component version. Encoded-parameter helpers return `None` when
  `ADD_ENCODED_PARAMS` is clear; they do not assume a FIFO depth.

```rust
use dw_apb_i2c::{Control, DataCommand, InterruptBits, Speed, TargetAddress};

let control = Control::default()
    .with_master_mode(true)
    .with_slave_disabled(true)
    .with_restart_enabled(true)
    .with_speed(Speed::Fast);
let target = TargetAddress::default().with_address(0x50);
let last_read = DataCommand::read_request().with_stop(true);
let mask = InterruptBits::default().with_events(InterruptBits::STOP_DETECT, true);

assert_eq!(control.bits(), 0x65);
assert_eq!(target.bits(), 0x50);
assert_eq!(last_read.bits(), 0x300);
assert_eq!(mask.bits(), 0x200);
```

Constructing values does not access hardware. Creating a register reference is
the caller's responsibility: the region must be aligned, mapped and accessible,
with clocks/reset established and no conflicting controller or DMA users.
An 8/16-bit APB or byte-swapped integration needs a different access layer.
`Default` means zero bits, not the hardware reset value or a working bus setup.

## References

Register offsets and access policies were cross-checked against these primary
implementation references; no SoC-specific reset values or FIFO depths are assumed:

- [Linux DesignWare core definitions](https://github.com/torvalds/linux/blob/master/drivers/i2c/busses/i2c-designware-core.h)
- [U-Boot DesignWare register layout](https://github.com/u-boot/u-boot/blob/master/drivers/i2c/designware_i2c.h)
- [Raspberry Pi RP2040 DesignWare register descriptions](https://github.com/raspberrypi/pico-sdk/blob/master/src/rp2040/hardware_regs/include/hardware/regs/i2c.h)

Host tests check layout, field encodings, bounds, capability decoding and exact
mock-memory accesses. Compile-fail doctests check that command and clear
registers cannot use the wrong access policy. They do not simulate an I²C bus.
