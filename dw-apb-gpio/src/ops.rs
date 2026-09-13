//! Register operations shared by GPIO interfaces.
//!
//! This module is carefully designed to minimize stack usage while preserving
//! correctness of the whole crate. Modify with care, double check for possible
//! stack usage regressions while you are modifying this module.

use core::{
    cell::UnsafeCell,
    mem::{align_of, size_of},
};
use critical_section::CriticalSection;
use embedded_hal::digital::PinState;

use volatile_register::{RO, RW, WO};

use crate::{eint::Event, eint_with_both_edges::EventWithBothEdges, pad::Pad, registers::PinBits};

// RO/RW/WO -> VolatileCell -> UnsafeCell -> PinBits -> u32 are transparent wrappers.
// Keep the volatile accesses at this boundary to avoid spilling each wrapper's
// arguments in debug builds. Obtain writable pointers through UnsafeCell::raw_get.
const _: () = {
    assert!(size_of::<PinBits>() == size_of::<u32>());
    assert!(align_of::<PinBits>() == align_of::<u32>());
    assert!(size_of::<RO<PinBits>>() == size_of::<u32>());
    assert!(align_of::<RO<PinBits>>() == align_of::<u32>());
    assert!(size_of::<RW<PinBits>>() == size_of::<u32>());
    assert!(align_of::<RW<PinBits>>() == align_of::<u32>());
    assert!(size_of::<WO<PinBits>>() == size_of::<u32>());
    assert!(align_of::<WO<PinBits>>() == align_of::<u32>());
};

#[inline(always)]
pub(crate) fn op_level(pad: &Pad<'_>) -> bool {
    // SAFETY: Pad guarantees accessible registers; the transparent wrappers preserve u32 alignment and layout.
    (unsafe {
        core::ptr::read_volatile(
            &pad.gpio.external_port[pad.port as usize] as *const RO<PinBits> as *const u32,
        )
    }) & pad.mask
        != 0
}

#[inline(always)]
pub(crate) fn op_latch(pad: &Pad<'_>) -> bool {
    // SAFETY: Pad guarantees accessible registers; the transparent wrappers preserve u32 alignment and layout.
    (unsafe {
        core::ptr::read_volatile(
            &pad.gpio.port[pad.port as usize].data as *const RW<PinBits> as *const u32,
        )
    }) & pad.mask
        != 0
}

#[inline(always)]
pub(crate) fn op_set_level(_cs: CriticalSection<'_>, pad: &Pad<'_>, state: PinState) {
    // SAFETY: the token serializes the RMW of this owned pin.
    unsafe {
        let register = &pad.gpio.port[pad.port as usize].data;
        match state {
            PinState::Low => write(register, read(register) & !pad.mask),
            PinState::High => write(register, read(register) | pad.mask),
        }
    }
}

#[inline(always)]
pub(crate) fn op_toggle(_cs: CriticalSection<'_>, pad: &Pad<'_>) {
    // SAFETY: the owned pin is accessible and the supplied critical-section token serializes the entire RMW.
    unsafe {
        write(
            &pad.gpio.port[pad.port as usize].data,
            read(&pad.gpio.port[pad.port as usize].data) ^ pad.mask,
        );
    }
}

#[inline(always)]
pub(crate) fn op_set_input(_cs: CriticalSection<'_>, pad: &Pad<'_>) {
    // SAFETY: the owned pin is accessible and the supplied critical-section token serializes the entire RMW sequence.
    unsafe {
        write(
            &pad.gpio.port[pad.port as usize].direction,
            read(&pad.gpio.port[pad.port as usize].direction) & !pad.mask,
        );
    }
}

#[inline(always)]
pub(crate) fn op_set_output(_cs: CriticalSection<'_>, pad: &Pad<'_>, state: PinState) {
    // SAFETY: the owned pin is accessible and the supplied critical-section token serializes the entire RMW sequence.
    unsafe {
        let port = &pad.gpio.port[pad.port as usize];
        match state {
            PinState::Low => write(&port.data, read(&port.data) & !pad.mask),
            PinState::High => write(&port.data, read(&port.data) | pad.mask),
        }
        write(&port.direction, read(&port.direction) | pad.mask);
    }
}

#[inline(always)]
pub(crate) fn op_into_eint(_cs: CriticalSection<'_>, pad: &Pad<'_>) {
    assert!(pad.port == 0, "only port A supports interrupts");

    // SAFETY: the owned pin is accessible and the supplied critical-section token serializes the entire RMW sequence.
    unsafe {
        write(
            &pad.gpio.interrupt_mask,
            read(&pad.gpio.interrupt_mask) | pad.mask,
        );
        write(
            &pad.gpio.interrupt_enable,
            read(&pad.gpio.interrupt_enable) & !pad.mask,
        );
        write(
            &pad.gpio.port[pad.port as usize].direction,
            read(&pad.gpio.port[pad.port as usize].direction) & !pad.mask,
        );
    }
}

#[inline(always)]
pub(crate) fn op_eint_into_output(_cs: CriticalSection<'_>, pad: &Pad<'_>, state: PinState) {
    // SAFETY: the owned pin is accessible and the supplied critical-section token serializes the entire RMW sequence.
    unsafe {
        write(
            &pad.gpio.interrupt_mask,
            read(&pad.gpio.interrupt_mask) | pad.mask,
        );
        write(
            &pad.gpio.interrupt_enable,
            read(&pad.gpio.interrupt_enable) & !pad.mask,
        );
        let port = &pad.gpio.port[pad.port as usize];
        match state {
            PinState::Low => write(&port.data, read(&port.data) & !pad.mask),
            PinState::High => write(&port.data, read(&port.data) | pad.mask),
        }
        write(&port.direction, read(&port.direction) | pad.mask);
    }
}

#[inline(always)]
pub(crate) fn op_enable_interrupt(_cs: CriticalSection<'_>, pad: &Pad<'_>) {
    // SAFETY: the owned pin is accessible and the supplied critical-section token serializes the entire RMW sequence.
    unsafe {
        write(
            &pad.gpio.interrupt_enable,
            read(&pad.gpio.interrupt_enable) | pad.mask,
        );
        write(
            &pad.gpio.interrupt_mask,
            read(&pad.gpio.interrupt_mask) & !pad.mask,
        );
    }
}

#[inline(always)]
pub(crate) fn op_disable_interrupt(_cs: CriticalSection<'_>, pad: &Pad<'_>) {
    // SAFETY: the owned pin is accessible and the supplied critical-section token serializes the entire RMW sequence.
    unsafe {
        write(
            &pad.gpio.interrupt_mask,
            read(&pad.gpio.interrupt_mask) | pad.mask,
        );
        write(
            &pad.gpio.interrupt_enable,
            read(&pad.gpio.interrupt_enable) & !pad.mask,
        );
    }
}

#[inline(always)]
pub(crate) fn op_clear_interrupt(pad: &Pad<'_>) {
    // SAFETY: the owned port A pin has an accessible WO register backed by UnsafeCell;
    // the transparent wrappers preserve u32 layout, and only this pin's W1C bit is written.
    unsafe {
        core::ptr::write_volatile(
            UnsafeCell::raw_get(
                &pad.gpio.interrupt_clear as *const WO<PinBits> as *const UnsafeCell<PinBits>,
            ) as *mut u32,
            pad.mask,
        )
    };
}

#[inline(always)]
pub(crate) fn op_interrupt_pending(pad: &Pad<'_>) -> bool {
    // SAFETY: Pad guarantees accessible registers; the transparent wrappers preserve u32 alignment and layout.
    (unsafe {
        core::ptr::read_volatile(&pad.gpio.interrupt_status as *const RO<PinBits> as *const u32)
    }) & pad.mask
        != 0
}

#[inline(always)]
pub(crate) fn op_set_event(_cs: CriticalSection<'_>, pad: &Pad<'_>, event: Event) {
    let gpio = pad.gpio;

    // SAFETY: the owned pin is accessible and the supplied critical-section token serializes the entire RMW sequence.
    unsafe {
        let masked = read(&gpio.interrupt_mask);
        write(&gpio.interrupt_mask, masked | pad.mask);
        if matches!(event, Event::RisingEdge | Event::FallingEdge) {
            write(&gpio.interrupt_type, read(&gpio.interrupt_type) | pad.mask);
        } else {
            write(&gpio.interrupt_type, read(&gpio.interrupt_type) & !pad.mask);
        }
        if matches!(event, Event::RisingEdge | Event::HighLevel) {
            write(
                &gpio.interrupt_polarity,
                read(&gpio.interrupt_polarity) | pad.mask,
            );
        } else {
            write(
                &gpio.interrupt_polarity,
                read(&gpio.interrupt_polarity) & !pad.mask,
            );
        }
        write(&gpio.interrupt_mask, masked);
    }
}

#[inline(always)]
pub(crate) fn op_set_event_with_both_edges(
    _cs: CriticalSection<'_>,
    pad: &Pad<'_>,
    event: EventWithBothEdges,
) {
    let gpio = pad.gpio;

    // SAFETY: the owned pin is accessible and the supplied critical-section token serializes the entire RMW sequence.
    unsafe {
        let masked = read(&gpio.interrupt_mask);
        write(&gpio.interrupt_mask, masked | pad.mask);
        if matches!(
            event,
            EventWithBothEdges::RisingEdge
                | EventWithBothEdges::FallingEdge
                | EventWithBothEdges::BothEdges
        ) {
            write(&gpio.interrupt_type, read(&gpio.interrupt_type) | pad.mask);
        } else {
            write(&gpio.interrupt_type, read(&gpio.interrupt_type) & !pad.mask);
        }
        if matches!(
            event,
            EventWithBothEdges::RisingEdge | EventWithBothEdges::HighLevel
        ) {
            write(
                &gpio.interrupt_polarity,
                read(&gpio.interrupt_polarity) | pad.mask,
            );
        } else {
            write(
                &gpio.interrupt_polarity,
                read(&gpio.interrupt_polarity) & !pad.mask,
            );
        }
        if matches!(event, EventWithBothEdges::BothEdges) {
            write(
                &gpio.interrupt_both_edge,
                read(&gpio.interrupt_both_edge) | pad.mask,
            );
        } else {
            write(
                &gpio.interrupt_both_edge,
                read(&gpio.interrupt_both_edge) & !pad.mask,
            );
        }
        write(&gpio.interrupt_mask, masked);
    }
}

#[inline]
pub(crate) fn op_save(pad: &Pad<'_>) -> u8 {
    // Bit 1 records direction; bit 0 records the output latch.
    let port = &pad.gpio.port[pad.port as usize];
    (((read(&port.direction) & pad.mask != 0) as u8) << 1)
        | ((read(&port.data) & pad.mask != 0) as u8)
}

#[inline(always)]
pub(crate) fn op_restore(_cs: CriticalSection<'_>, pad: &Pad<'_>, state: u8) {
    // SAFETY: the owned pin is accessible and the supplied critical-section token serializes the entire RMW sequence.
    unsafe {
        let port = &pad.gpio.port[pad.port as usize];
        if state & 2 != 0 {
            if state & 1 != 0 {
                write(&port.data, read(&port.data) | pad.mask);
            } else {
                write(&port.data, read(&port.data) & !pad.mask);
            }
            write(&port.direction, read(&port.direction) | pad.mask);
        } else {
            write(&port.direction, read(&port.direction) & !pad.mask);
            if state & 1 != 0 {
                write(&port.data, read(&port.data) | pad.mask);
            } else {
                write(&port.data, read(&port.data) & !pad.mask);
            }
        }
    }
}

/// Reads the register word without the volatile-register/vcell accessor chain.
///
/// This reduced the helper's own frame in the default debug
/// profile on some architectures; measured release paths inline it.
/// Keep this shared helper with ordinary `#[inline]`; remeasure both profiles before changing it.
#[inline]
fn read(register: &RW<PinBits>) -> u32 {
    // SAFETY: the reference is valid for the read, and the transparent wrappers preserve u32 layout.
    unsafe { core::ptr::read_volatile(register as *const RW<PinBits> as *const u32) }
}

/// Writes the register word without the volatile-register/vcell accessor chain.
///
/// This reduced the helper's own frame in the default debug
/// profile on some architectures; measured release paths inline it.
/// Sharing this helper can keep callers smaller; remeasure both profiles before
/// forcing inlining or removing it.
#[inline]
unsafe fn write(register: &RW<PinBits>, bits: u32) {
    // SAFETY: the transparent wrappers place UnsafeCell<PinBits> at offset zero. raw_get
    // permits interior mutation; callers guarantee writable registers and serialize the RMW.
    unsafe {
        core::ptr::write_volatile(
            UnsafeCell::raw_get(register as *const RW<PinBits> as *const UnsafeCell<PinBits>)
                as *mut u32,
            bits,
        )
    };
}
