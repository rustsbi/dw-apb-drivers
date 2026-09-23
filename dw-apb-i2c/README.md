# DW APB I²C

`no_std` register layer for Synopsys DW_apb_i2c with little-endian 32-bit MMIO
accesses.

## Access semantics

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

## References

- **Synopsys DW_apb_i2c Databook, 2.05a**: [official directory](https://www.synopsys.com/dw/ipdir.php?c=DW_apb_i2c),
- **Raspberry Pi RP2040 Datasheet**, build 3184e62-clean, 2025-02-20 —
  [§4.3.17, List of Registers](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf#page=465)
  for register/bit-field names and access types;
  §4.3.16, Operation of Interrupt Registers (table 451), for read-clear semantics;
  §4.3.15, DMA Controller Interface, for `RDMAE`/`TDMAE`.
- **Intel Core Ultra Series 3 Processors I/O Registers**, document 872352,
  revision 001, 2026-04-14:
  [I2C MMIO, REG IC_CON and the linked register chapters](https://edc.intel.com/content/www/de/de/design/publications/core-ultra-series-3-processors-i-o-registers/reg-ic-con-ic-con-offset-0/),
  including `IC_HS_MADDR`, `IC_HS_SCL_HCNT`, `IC_HS_SCL_LCNT` and `IC_HS_SPKLEN`.
