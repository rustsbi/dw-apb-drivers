//! Synopsys DesignWare APB GPIO registers.

#![no_std]
#![deny(missing_docs)]

use volatile_register::{RO, RW, WO};

// Synopsys DW_apb_gpio Databook 2.11a (June 2015), table 6-1, sections 6.3, 7.3 and 9.1.
// This is the maximum register layout; access only the features implemented by the SoC.
/// DW_apb_gpio layout for little-endian 32-bit MMIO; register availability depends on IP parameters.
#[repr(C)]
pub struct DwApbGpio {
    /// `0x00..0x30`: software port registers in A, B, C, D order.
    pub port: [Port; 4],
    /// `0x30 GPIO_INTEN`: one enables a port A input interrupt in software mode.
    #[doc(alias = "GPIO_INTEN")]
    pub interrupt_enable: RW<PinBits>,
    /// `0x34 GPIO_INTMASK`: one masks a port A interrupt.
    #[doc(alias = "GPIO_INTMASK")]
    pub interrupt_mask: RW<PinBits>,
    /// `0x38 GPIO_INTTYPE_LEVEL`: zero selects level, one selects edge.
    #[doc(alias = "GPIO_INTTYPE_LEVEL")]
    pub interrupt_type: RW<PinBits>,
    /// `0x3c GPIO_INT_POLARITY`: zero selects low/falling, one high/rising.
    #[doc(alias = "GPIO_INT_POLARITY")]
    pub interrupt_polarity: RW<PinBits>,
    /// `0x40 GPIO_INTSTATUS`: pending port A interrupts after masking.
    #[doc(alias = "GPIO_INTSTATUS")]
    pub interrupt_status: RO<PinBits>,
    /// `0x44 GPIO_RAW_INTSTATUS`: pending port A interrupts before masking.
    #[doc(alias = "GPIO_RAW_INTSTATUS")]
    pub raw_interrupt_status: RO<PinBits>,
    /// `0x48 GPIO_DEBOUNCE`: optional port A input debounce, enabled by one.
    #[doc(alias = "GPIO_DEBOUNCE")]
    pub debounce: RW<PinBits>,
    /// `0x4c GPIO_PORTA_EOI`: write one to clear a port A edge interrupt; level interrupts are unaffected.
    #[doc(alias = "GPIO_PORTA_EOI")]
    pub interrupt_clear: WO<PinBits>,
    /// `0x50..0x60 GPIO_EXT_PORTx`: external pin levels in A, B, C, D order; synchronization is optional.
    #[doc(alias("GPIO_EXT_PORTA", "GPIO_EXT_PORTB", "GPIO_EXT_PORTC", "GPIO_EXT_PORTD"))]
    pub external_port: [RO<PinBits>; 4],
    /// `0x60 GPIO_LS_SYNC`: level interrupt synchronization; read-only when `GPIO_PORTA_INTR = 0`.
    #[doc(alias = "GPIO_LS_SYNC")]
    pub level_sync: RW<LevelSync>,
    /// `0x64 GPIO_ID_CODE`: optional integration-specific identifier.
    #[doc(alias = "GPIO_ID_CODE")]
    pub id_code: RO<u32>,
    /// `0x68 GPIO_INT_BOTHEDGE`: optional both-edge detection; one overrides type and polarity.
    #[doc(alias = "GPIO_INT_BOTHEDGE")]
    pub interrupt_both_edge: RW<PinBits>,
    /// `0x6c GPIO_VER_ID_CODE`: component version in ASCII.
    #[doc(alias = "GPIO_VER_ID_CODE")]
    pub version: RO<u32>,
    /// `0x70 GPIO_CONFIG_REG2`: port widths minus one; reads zero when encoded parameters are disabled.
    #[doc(alias = "GPIO_CONFIG_REG2")]
    pub configuration2: RO<u32>,
    /// `0x74 GPIO_CONFIG_REG1`: capabilities; reads zero when encoded parameters are disabled.
    #[doc(alias = "GPIO_CONFIG_REG1")]
    pub configuration1: RO<u32>,
}

/// Software port register group, repeated with a `0x0c` byte stride.
#[repr(C)]
pub struct Port {
    /// `+0x00 GPIO_SWPORTx_DR`: output latch; reads return the last written value.
    #[doc(alias(
        "GPIO_SWPORTA_DR",
        "GPIO_SWPORTB_DR",
        "GPIO_SWPORTC_DR",
        "GPIO_SWPORTD_DR"
    ))]
    pub data: RW<PinBits>,
    /// `+0x04 GPIO_SWPORTx_DDR`: zero selects input, one selects output.
    #[doc(alias(
        "GPIO_SWPORTA_DDR",
        "GPIO_SWPORTB_DDR",
        "GPIO_SWPORTC_DDR",
        "GPIO_SWPORTD_DDR"
    ))]
    pub direction: RW<PinBits>,
    /// `+0x08 GPIO_SWPORTx_CTL`: zero software, one hardware; `SINGLE_CTL` makes bit 0 control the whole port.
    #[doc(alias(
        "GPIO_SWPORTA_CTL",
        "GPIO_SWPORTB_CTL",
        "GPIO_SWPORTC_CTL",
        "GPIO_SWPORTD_CTL"
    ))]
    // GPIO_HW_PORTx enables this register; GPIO_PORTx_SINGLE_CTL selects one bit for the whole port.
    pub control: RW<u32>,
}

/// One bit per pin in a data, direction, debounce or interrupt register.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct PinBits(u32);

impl PinBits {
    /// Construct a value from raw bits, without validating the integrated port width.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw register bits.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Read a pin's bit; panics if `pin >= 32`.
    #[inline]
    pub const fn is_set(self, pin: u8) -> bool {
        assert!(pin < 32, "pin is outside the register");
        self.0 & (1 << pin) != 0
    }

    /// Set a pin's bit while preserving the others; panics if `pin >= 32`.
    #[inline]
    pub const fn with_pin(self, pin: u8, value: bool) -> Self {
        assert!(pin < 32, "pin is outside the register");
        let mask = 1 << pin;
        Self((self.0 & !mask) | if value { mask } else { 0 })
    }
}

/// Level-sensitive interrupt synchronization control (`GPIO_LS_SYNC`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct LevelSync(u32);

impl LevelSync {
    /// Construct a value from raw register bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Return the raw register bits.
    #[inline]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Whether level-sensitive interrupts are synchronized to `pclk_intr`.
    #[inline]
    pub const fn is_enabled(self) -> bool {
        self.0 & 1 != 0
    }

    /// Enable or disable synchronization while preserving the other bits.
    #[inline]
    pub const fn with_enabled(self, enabled: bool) -> Self {
        Self((self.0 & !1) | enabled as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    #[test]
    fn register_layout() {
        assert_eq!(size_of::<PinBits>(), 4);
        assert_eq!(size_of::<LevelSync>(), 4);
        assert_eq!(size_of::<Port>(), 0x0c);
        assert_eq!(align_of::<Port>(), 4);
        assert_eq!(offset_of!(Port, data), 0x00);
        assert_eq!(offset_of!(Port, direction), 0x04);
        assert_eq!(offset_of!(Port, control), 0x08);
        for (index, data, direction, control, external) in [
            (0, 0x00, 0x04, 0x08, 0x50),
            (1, 0x0c, 0x10, 0x14, 0x54),
            (2, 0x18, 0x1c, 0x20, 0x58),
            (3, 0x24, 0x28, 0x2c, 0x5c),
        ] {
            let base = offset_of!(DwApbGpio, port) + index * size_of::<Port>();
            assert_eq!(base + offset_of!(Port, data), data);
            assert_eq!(base + offset_of!(Port, direction), direction);
            assert_eq!(base + offset_of!(Port, control), control);
            assert_eq!(
                offset_of!(DwApbGpio, external_port) + index * size_of::<RO<PinBits>>(),
                external
            );
        }
        assert_eq!(offset_of!(DwApbGpio, interrupt_enable), 0x30);
        assert_eq!(offset_of!(DwApbGpio, interrupt_mask), 0x34);
        assert_eq!(offset_of!(DwApbGpio, interrupt_type), 0x38);
        assert_eq!(offset_of!(DwApbGpio, interrupt_polarity), 0x3c);
        assert_eq!(offset_of!(DwApbGpio, interrupt_status), 0x40);
        assert_eq!(offset_of!(DwApbGpio, raw_interrupt_status), 0x44);
        assert_eq!(offset_of!(DwApbGpio, debounce), 0x48);
        assert_eq!(offset_of!(DwApbGpio, interrupt_clear), 0x4c);
        assert_eq!(offset_of!(DwApbGpio, external_port), 0x50);
        assert_eq!(offset_of!(DwApbGpio, level_sync), 0x60);
        assert_eq!(offset_of!(DwApbGpio, id_code), 0x64);
        assert_eq!(offset_of!(DwApbGpio, interrupt_both_edge), 0x68);
        assert_eq!(offset_of!(DwApbGpio, version), 0x6c);
        assert_eq!(offset_of!(DwApbGpio, configuration2), 0x70);
        assert_eq!(offset_of!(DwApbGpio, configuration1), 0x74);
        assert_eq!(size_of::<DwApbGpio>(), 0x78);
        assert_eq!(align_of::<DwApbGpio>(), 4);
    }

    #[test]
    fn bit_values_and_bounds() {
        let original = PinBits::from_bits(0xa5a5_a5a5);
        assert_eq!(original.bits(), 0xa5a5_a5a5);
        assert!(original.is_set(0));
        assert!(!original.is_set(1));
        assert!(original.is_set(31));
        assert_eq!(original.with_pin(31, false).bits(), 0x25a5_a5a5);
        assert_eq!(original.with_pin(1, true).bits(), 0xa5a5_a5a7);
        assert_eq!(original.with_pin(0, false).bits(), 0xa5a5_a5a4);
        assert_eq!(PinBits::default().with_pin(31, true).bits(), 0x8000_0000);
        for pin in [32, 255] {
            assert!(std::panic::catch_unwind(|| original.is_set(pin)).is_err());
            assert!(std::panic::catch_unwind(|| original.with_pin(pin, true)).is_err());
        }
        let sync = LevelSync::from_bits(0xa5a5_a5a5);
        assert_eq!(sync.bits(), 0xa5a5_a5a5);
        assert!(sync.is_enabled());
        assert!(!sync.with_enabled(false).is_enabled());
        assert_eq!(sync.with_enabled(false).bits(), 0xa5a5_a5a4);
        assert!(!LevelSync::default().is_enabled());
        assert_eq!(LevelSync::default().with_enabled(true).bits(), 1);
    }

    extern crate std;
}
