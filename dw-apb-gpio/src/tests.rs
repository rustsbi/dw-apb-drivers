extern crate std;

use crate::{
    eint::{EintPad, Event},
    eint_with_both_edges::{EintPadWithBothEdges, EventWithBothEdges},
    flex_pad::FlexPad,
    input::Input,
    output::Output,
    registers::DwApbGpio,
};
use core::cell::UnsafeCell;
use embedded_hal::digital::PinState;
use std::panic::{AssertUnwindSafe, catch_unwind};

struct Mock {
    words: [UnsafeCell<u32>; 0x78 / 4],
}

impl Mock {
    fn new() -> Self {
        Self {
            words: core::array::from_fn(|_| UnsafeCell::new(0)),
        }
    }

    fn gpio(&self) -> &DwApbGpio {
        // SAFETY: the layout tests verify the size/alignment and offsets;
        // every register wraps an UnsafeCell<u32> and zero is a valid value.
        unsafe { &*self.words.as_ptr().cast::<DwApbGpio>() }
    }

    fn read(&self, offset: usize) -> u32 {
        // SAFETY: tests are single-threaded with respect to each mock.
        unsafe { self.words[offset / 4].get().read_volatile() }
    }

    fn write(&self, offset: usize, value: u32) {
        // SAFETY: the mock uses interior mutability and has no concurrent users.
        unsafe { self.words[offset / 4].get().write_volatile(value) };
    }
}

#[test]
fn digital_traits_use_external_levels_and_output_latches() {
    let mock = Mock::new();
    // SAFETY: these are distinct pins on accessible mock registers.
    let mut input = unsafe { Input::new('B', 31, mock.gpio()) };
    let mut output = unsafe { Output::new('D', 7, mock.gpio(), PinState::High) };
    mock.write(0x0c, 1 << 31);
    let shared_input = &input;
    assert!(!shared_input.is_high());
    assert!(shared_input.is_low());
    assert_eq!(
        embedded_hal::digital::InputPin::is_high(&mut input),
        Ok(false)
    );
    mock.write(0x54, 1 << 31);
    assert!(input.is_high());
    assert!(!input.is_low());
    assert_eq!(
        embedded_hal::digital::InputPin::is_high(&mut input),
        Ok(true)
    );
    assert_eq!(
        embedded_hal::digital::InputPin::is_low(&mut input),
        Ok(false)
    );
    assert_eq!(mock.read(0x28), 1 << 7);
    assert_eq!(mock.read(0x5c), 0);
    let shared_output = &output;
    assert!(shared_output.is_set_high());
    assert!(!shared_output.is_set_low());
    assert_eq!(
        embedded_hal::digital::StatefulOutputPin::is_set_high(&mut output),
        Ok(true)
    );
    embedded_hal::digital::OutputPin::set_low(&mut output).unwrap();
    assert_eq!(
        embedded_hal::digital::StatefulOutputPin::is_set_low(&mut output),
        Ok(true)
    );
    embedded_hal::digital::StatefulOutputPin::toggle(&mut output).unwrap();
    assert_eq!(
        embedded_hal::digital::StatefulOutputPin::is_set_high(&mut output),
        Ok(true)
    );
    embedded_hal::digital::OutputPin::set_state(&mut output, PinState::Low).unwrap();
    assert_eq!(mock.read(0x24), 0);
    output.set_level(PinState::High);
    assert_eq!(mock.read(0x24), 1 << 7);
    output.set_level(PinState::Low);
    assert_eq!(mock.read(0x24), 0);
}

#[test]
fn cached_masks_preserve_neighbours_on_every_port() {
    for (port, index) in [('A', 0), ('B', 1), ('C', 2), ('D', 3)] {
        for number in 0..32 {
            let mock = Mock::new();
            let mask = 1u32 << number;
            let data = index * 0x0c;
            let direction = data + 4;
            let external = 0x50 + index * 4;
            mock.write(data, 0xa5a5_a5a5);
            mock.write(direction, 0x5a5a_5a5a);
            // SAFETY: this is the only acquired pin in this accessible mock.
            let mut output = unsafe { Output::new(port, number, mock.gpio(), PinState::Low) };
            assert_eq!(mock.read(data), 0xa5a5_a5a5 & !mask);
            assert_eq!(mock.read(direction), 0x5a5a_5a5a | mask);
            output.set_high();
            assert_eq!(mock.read(data), 0xa5a5_a5a5 | mask);
            let mut flex = FlexPad::from(output);
            mock.write(external, mask);
            assert!(flex.is_high());
            flex.toggle();
            assert!(flex.is_set_low());
            assert!(flex.is_high());
            assert_eq!(mock.read(data), 0xa5a5_a5a5 & !mask);
            let input = flex.into_input();
            assert!(input.is_high());
            assert_eq!(mock.read(direction), 0x5a5a_5a5a & !mask);
        }
    }
}

#[test]
fn transitions_preserve_other_pins() {
    let mock = Mock::new();
    mock.write(0, 1 << 9);
    mock.write(4, 1 << 9);
    // SAFETY: pin 3 is uniquely owned on the mock.
    let input = unsafe { Input::new('A', 3, mock.gpio()) };
    let output = input.into_output(PinState::High);
    assert_eq!(mock.read(0), (1 << 9) | (1 << 3));
    assert_eq!(mock.read(4), (1 << 9) | (1 << 3));
    let mut flex = FlexPad::from(output.into_input());
    assert_eq!(mock.read(4), 1 << 9);
    flex.set_as_output(PinState::Low);
    assert_eq!(mock.read(0), 1 << 9);
    flex.set_high();
    assert!(flex.is_set_high());
    flex.set_as_input();
    assert_eq!(mock.read(4), 1 << 9);
    // Software/hardware control belongs to SoC integration, not direction changes.
    assert_eq!(mock.read(8), 0);
}

#[test]
fn temporary_modes_restore_on_return_and_unwind() {
    let mock = Mock::new();
    // SAFETY: these are distinct pins on accessible mock registers.
    let mut input = unsafe { Input::new('C', 2, mock.gpio()) };
    let mut neighbour = unsafe { Output::new('C', 3, mock.gpio(), PinState::Low) };
    let result = input.with_output(PinState::High, |output| {
        assert_eq!(mock.read(0x1c), (1 << 2) | (1 << 3));
        assert!(output.is_set_high());
        neighbour.set_high();
        42
    });
    assert_eq!(result, 42);
    assert_eq!(mock.read(0x1c), 1 << 3);
    assert_eq!(mock.read(0x18), 1 << 3);
    let panic = catch_unwind(AssertUnwindSafe(|| {
        input.with_output(PinState::High, |_| panic!("test unwind"));
    }));
    assert!(panic.is_err());
    assert_eq!(mock.read(0x1c), 1 << 3);
    assert_eq!(mock.read(0x18), 1 << 3);
    let panic = catch_unwind(AssertUnwindSafe(|| {
        neighbour.with_input(|_| {
            assert_eq!(mock.read(0x1c), 0);
            panic!("test unwind");
        });
    }));
    assert!(panic.is_err());
    assert_eq!(mock.read(0x1c), 1 << 3);
    assert!(neighbour.is_set_high());
}

#[test]
fn interrupt_events_masking_and_teardown() {
    let mock = Mock::new();
    mock.write(0x30, 1 << 8);
    mock.write(0x34, 1 << 8);
    mock.write(0x38, 1 << 8);
    mock.write(0x3c, 1 << 8);
    mock.write(0x68, 1 << 8);
    // SAFETY: pin 5 is unique and all interrupt registers exist in the mock.
    let mut eint = unsafe { EintPadWithBothEdges::new(5, mock.gpio()) };
    assert_eq!(mock.read(0x30), 1 << 8);
    assert_eq!(mock.read(0x34), (1 << 8) | (1 << 5));
    for (event, edge, high, both) in [
        (EventWithBothEdges::RisingEdge, true, true, false),
        (EventWithBothEdges::FallingEdge, true, false, false),
        (EventWithBothEdges::HighLevel, false, true, false),
        (EventWithBothEdges::LowLevel, false, false, false),
        (EventWithBothEdges::BothEdges, true, false, true),
        (EventWithBothEdges::RisingEdge, true, true, false),
    ] {
        eint.set_event(event);
        assert_eq!(mock.read(0x38), (1 << 8) | ((edge as u32) << 5));
        assert_eq!(mock.read(0x3c), (1 << 8) | ((high as u32) << 5));
        assert_eq!(mock.read(0x68), (1 << 8) | ((both as u32) << 5));
    }
    eint.enable_interrupt();
    assert_eq!(mock.read(0x30), (1 << 8) | (1 << 5));
    assert_eq!(mock.read(0x34), 1 << 8);
    eint.set_event(EventWithBothEdges::LowLevel);
    assert_eq!(mock.read(0x34), 1 << 8);
    assert!(!eint.is_interrupt_pending());
    mock.write(0x40, 1 << 5);
    assert!(eint.is_interrupt_pending());
    eint.clear_interrupt();
    assert_eq!(mock.read(0x4c), 1 << 5);
    let output = eint.into_output(PinState::High);
    assert_eq!(mock.read(0x30), 1 << 8);
    assert_eq!(mock.read(0x34), (1 << 8) | (1 << 5));
    assert!(output.is_set_high());
}

#[test]
fn single_edge_events_never_touch_both_edge_register() {
    let mock = Mock::new();
    // SAFETY: pin 0 is unique; this integration omits hardware both-edge detection.
    let mut eint = unsafe { EintPad::new(0, mock.gpio()) };
    mock.write(0x68, 0xfeed_beef);
    mock.write(0x38, 1 << 8);
    mock.write(0x3c, 1 << 8);
    for (event, edge, high) in [
        (Event::RisingEdge, true, true),
        (Event::FallingEdge, true, false),
        (Event::HighLevel, false, true),
        (Event::LowLevel, false, false),
    ] {
        eint.set_event(event);
        assert_eq!(mock.read(0x38), (1 << 8) | edge as u32);
        assert_eq!(mock.read(0x3c), (1 << 8) | high as u32);
        assert_eq!(mock.read(0x68), 0xfeed_beef);
    }
    eint.enable_interrupt();
    assert_eq!(mock.read(0x30), 1);
    assert_eq!(mock.read(0x34), 0);
    mock.write(0x40, 1);
    assert!(eint.is_interrupt_pending());
    eint.clear_interrupt();
    assert_eq!(mock.read(0x4c), 1);
    let output = eint.into_output(PinState::High);
    assert!(output.is_set_high());
    assert_eq!(mock.read(0x30), 0);
    assert_eq!(mock.read(0x34), 1);
    assert_eq!(mock.read(0x68), 0xfeed_beef);
}

#[test]
fn invalid_port_and_pin_are_rejected_before_access() {
    let mock = Mock::new();
    for (port, number) in [('a', 0), ('E', 0), ('A', 32), ('D', 255)] {
        // SAFETY: invalid identifiers are rejected before accessing registers.
        assert!(
            catch_unwind(AssertUnwindSafe(|| unsafe {
                FlexPad::new(port, number, mock.gpio())
            }))
            .is_err()
        );
    }
    // SAFETY: port B exists; into_eint rejects it before touching interrupt registers.
    assert!(
        catch_unwind(AssertUnwindSafe(|| unsafe {
            FlexPad::new('B', 0, mock.gpio()).into_eint()
        }))
        .is_err()
    );
    // SAFETY: the invalid port is rejected before any interrupt register access.
    assert!(
        catch_unwind(AssertUnwindSafe(|| unsafe {
            FlexPad::new('B', 0, mock.gpio()).into_eint_with_both_edges()
        }))
        .is_err()
    );
    for i in 0..0x78 / 4 {
        assert_eq!(mock.read(i * 4), 0);
    }
}

#[test]
fn both_interrupt_types_support_mode_round_trips() {
    let single = Mock::new();
    // SAFETY: this mock pin is unique and models an integration without both-edge detection.
    let pin = unsafe { FlexPad::new('A', 2, single.gpio()) };
    let input = pin.into_input();
    // SAFETY: the same integration guarantee holds after mode changes.
    let eint = unsafe { input.into_eint() };
    let output = eint.into_output(PinState::High);
    // SAFETY: the owned port A pin still belongs to the single-edge integration.
    let eint = unsafe { output.into_eint() };
    let mut flex = FlexPad::from(eint);
    flex.set_as_output(PinState::Low);
    assert!(flex.is_set_low());
    assert_eq!(single.read(0x30), 0);

    let both = Mock::new();
    // SAFETY: this distinct mock implements the both-edge register and owns pin 2.
    let pin = unsafe { FlexPad::new('A', 2, both.gpio()) };
    let input = pin.into_input();
    // SAFETY: the integration implements hardware both-edge detection.
    let mut eint = unsafe { input.into_eint_with_both_edges() };
    eint.set_event(EventWithBothEdges::BothEdges);
    assert_eq!(both.read(0x68), 1 << 2);
    let output = eint.into_output(PinState::High);
    // SAFETY: the same integration guarantee holds after the output conversion.
    let mut eint = unsafe { output.into_eint_with_both_edges() };
    eint.set_event(EventWithBothEdges::HighLevel);
    assert_eq!(both.read(0x68), 0);
    let mut flex = FlexPad::from(eint);
    flex.set_as_output(PinState::Low);
    assert!(flex.is_set_low());
    assert_eq!(both.read(0x30), 0);
}

#[test]
fn temporary_modes_restore_both_latch_levels_and_preserve_neighbours() {
    let owned = 1u32 << 31;
    let adjacent = 1u32 << 7;
    for initial in [PinState::Low, PinState::High] {
        for unwind in [false, true] {
            let mock = Mock::new();
            // SAFETY: the two pins are distinct and exclusively acquired on this mock.
            let mut output = unsafe { Output::new('B', 31, mock.gpio(), initial) };
            let mut neighbour = unsafe { Output::new('B', 7, mock.gpio(), PinState::Low) };
            let expected = if initial == PinState::High { owned } else { 0 };
            let result = catch_unwind(AssertUnwindSafe(|| {
                output.with_input(|_| {
                    assert_eq!(mock.read(0x10) & owned, 0);
                    neighbour.set_high();
                    assert!(!unwind, "test temporary input unwind");
                    42
                })
            }));
            assert_eq!(result.is_err(), unwind);
            if let Ok(value) = result {
                assert_eq!(value, 42);
            }
            assert_eq!(mock.read(0x10), owned | adjacent);
            assert_eq!(mock.read(0x0c), expected | adjacent);

            let mut input = output.into_input();
            let temporary = if initial == PinState::High {
                PinState::Low
            } else {
                PinState::High
            };
            let result = catch_unwind(AssertUnwindSafe(|| {
                input.with_output(temporary, |pin| {
                    assert_eq!(mock.read(0x10) & owned, owned);
                    assert_eq!(pin.is_set_high(), temporary == PinState::High);
                    neighbour.set_low();
                    assert!(!unwind, "test temporary output unwind");
                    24
                })
            }));
            assert_eq!(result.is_err(), unwind);
            if let Ok(value) = result {
                assert_eq!(value, 24);
            }
            assert_eq!(mock.read(0x10), adjacent);
            assert_eq!(mock.read(0x0c), expected);
        }
    }
}
