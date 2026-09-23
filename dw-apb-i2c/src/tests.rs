extern crate std;

use crate::*;
use core::mem::{align_of, offset_of, size_of};

#[test]
fn register_layout() {
    assert_eq!(offset_of!(DwApbI2c, control), 0x0);
    assert_eq!(offset_of!(DwApbI2c, target_address), 0x4);
    assert_eq!(offset_of!(DwApbI2c, slave_address), 0x8);
    assert_eq!(offset_of!(DwApbI2c, data_command), 0x10);
    assert_eq!(offset_of!(DwApbI2c, standard_scl_high), 0x14);
    assert_eq!(offset_of!(DwApbI2c, standard_scl_low), 0x18);
    assert_eq!(offset_of!(DwApbI2c, fast_scl_high), 0x1c);
    assert_eq!(offset_of!(DwApbI2c, fast_scl_low), 0x20);
    assert_eq!(offset_of!(DwApbI2c, high_scl_high), 0x24);
    assert_eq!(offset_of!(DwApbI2c, high_scl_low), 0x28);
    assert_eq!(offset_of!(DwApbI2c, interrupt_status), 0x2c);
    assert_eq!(offset_of!(DwApbI2c, interrupt_mask), 0x30);
    assert_eq!(offset_of!(DwApbI2c, raw_interrupt_status), 0x34);
    assert_eq!(offset_of!(DwApbI2c, receive_threshold), 0x38);
    assert_eq!(offset_of!(DwApbI2c, transmit_threshold), 0x3c);
    assert_eq!(offset_of!(DwApbI2c, clear_interrupts), 0x40);
    assert_eq!(offset_of!(DwApbI2c, clear_receive_underflow), 0x44);
    assert_eq!(offset_of!(DwApbI2c, clear_receive_overflow), 0x48);
    assert_eq!(offset_of!(DwApbI2c, clear_transmit_overflow), 0x4c);
    assert_eq!(offset_of!(DwApbI2c, clear_read_request), 0x50);
    assert_eq!(offset_of!(DwApbI2c, clear_transmit_abort), 0x54);
    assert_eq!(offset_of!(DwApbI2c, clear_receive_done), 0x58);
    assert_eq!(offset_of!(DwApbI2c, clear_activity), 0x5c);
    assert_eq!(offset_of!(DwApbI2c, clear_stop_detect), 0x60);
    assert_eq!(offset_of!(DwApbI2c, clear_start_detect), 0x64);
    assert_eq!(offset_of!(DwApbI2c, clear_general_call), 0x68);
    assert_eq!(offset_of!(DwApbI2c, enable), 0x6c);
    assert_eq!(offset_of!(DwApbI2c, status), 0x70);
    assert_eq!(offset_of!(DwApbI2c, transmit_fifo_level), 0x74);
    assert_eq!(offset_of!(DwApbI2c, receive_fifo_level), 0x78);
    assert_eq!(offset_of!(DwApbI2c, sda_hold), 0x7c);
    assert_eq!(offset_of!(DwApbI2c, transmit_abort_source), 0x80);
    assert_eq!(offset_of!(DwApbI2c, slave_data_nack_only), 0x84);
    assert_eq!(offset_of!(DwApbI2c, dma_control), 0x88);
    assert_eq!(offset_of!(DwApbI2c, dma_transmit_level), 0x8c);
    assert_eq!(offset_of!(DwApbI2c, dma_receive_level), 0x90);
    assert_eq!(offset_of!(DwApbI2c, sda_setup), 0x94);
    assert_eq!(offset_of!(DwApbI2c, acknowledge_general_call), 0x98);
    assert_eq!(offset_of!(DwApbI2c, enable_status), 0x9c);
    assert_eq!(offset_of!(DwApbI2c, fast_spike_length), 0xa0);
    assert_eq!(offset_of!(DwApbI2c, clear_restart_detect), 0xa8);
    assert_eq!(offset_of!(DwApbI2c, component_parameters), 0xf4);
    assert_eq!(offset_of!(DwApbI2c, component_version), 0xf8);
    assert_eq!(offset_of!(DwApbI2c, component_type), 0xfc);
    assert_eq!(size_of::<DwApbI2c>(), 0x100);
    assert_eq!(align_of::<DwApbI2c>(), 4);
    assert_eq!(size_of::<ReadClear>(), 4);
    assert_eq!(size_of::<DataCommandRegister>(), 4);
    assert_eq!(size_of::<ReadWriteCommand<Enable>>(), 4);
}

#[test]
fn optional_high_speed_layout() {
    assert_eq!(offset_of!(DwApbI2c, high_speed_master_code), 0x0c);
    assert_eq!(offset_of!(DwApbI2c, high_spike_length), 0xa4);
}

#[test]
fn control_encodings_preserve_unrelated_bits() {
    let original = Control::from_bits(0xa5a5_ffff);
    for (speed, encoding) in [(Speed::Standard, 2), (Speed::Fast, 4), (Speed::High, 6)] {
        let value = original.with_speed(speed);
        assert_eq!(value.speed(), Some(speed));
        assert_eq!(value.bits(), (original.bits() & !6) | encoding);
    }
    assert_eq!(Control::default().speed(), None);
    let control = Control::default()
        .with_master_mode(true)
        .with_speed(Speed::Fast)
        .with_restart_enabled(true)
        .with_slave_disabled(true);
    assert_eq!(control.bits(), 0x65);
    assert!(control.master_mode());
    assert!(control.restart_enabled());
    assert!(control.slave_disabled());
    macro_rules! check_bit {
        ($setter:ident, $getter:ident, $bit:expr) => {
            for bits in [0u32, u32::MAX, 0xa5a5_a5a5, 0x5a5a_5a5a] {
                for enabled in [false, true] {
                    let value = Control::from_bits(bits).$setter(enabled);
                    assert_eq!(value.$getter(), enabled);
                    assert_eq!(
                        value.bits(),
                        (bits & !(1 << $bit)) | ((enabled as u32) << $bit)
                    );
                }
            }
        };
    }
    check_bit!(with_master_mode, master_mode, 0);
    check_bit!(with_slave_ten_bit_addressing, slave_ten_bit_addressing, 3);
    check_bit!(with_master_ten_bit_addressing, master_ten_bit_addressing, 4);
    check_bit!(with_restart_enabled, restart_enabled, 5);
    check_bit!(with_slave_disabled, slave_disabled, 6);
    check_bit!(with_stop_detect_if_addressed, stop_detect_if_addressed, 7);
    check_bit!(with_transmit_empty_control, transmit_empty_control, 8);
    check_bit!(with_receive_fifo_full_hold, receive_fifo_full_hold, 9);
    check_bit!(
        with_stop_detect_if_master_active,
        stop_detect_if_master_active,
        10
    );
    check_bit!(with_bus_clear_control, bus_clear_control, 11);
}

#[test]
fn addresses_and_hold_counts_preserve_neighbours() {
    let target = TargetAddress::from_bits(0xffff_ffff);
    for address in [0, 0x7f, 0x80, 0x3ff] {
        assert_eq!(target.with_address(address).address(), address);
        assert_eq!(
            target.with_address(address).bits(),
            0xffff_fc00 | u32::from(address)
        );
    }
    assert!(std::panic::catch_unwind(|| target.with_address(0x400)).is_err());
    assert!(std::panic::catch_unwind(|| target.with_address(u16::MAX)).is_err());
    assert_eq!(target.with_start_byte(false).bits(), 0xffff_fbff);
    assert_eq!(target.with_special(false).bits(), 0xffff_f7ff);
    assert_eq!(target.with_ten_bit_addressing(false).bits(), 0xffff_efff);
    assert!(target.start_byte());
    assert!(target.special());
    assert!(target.ten_bit_addressing());
    let hold = SdaHold::from_bits(0xa5ff_ffff)
        .with_transmit(0x1234)
        .with_receive(0x56);
    assert_eq!(hold.bits(), 0xa556_1234);
    assert_eq!(hold.transmit(), 0x1234);
    assert_eq!(hold.receive(), 0x56);
}

#[test]
fn commands_status_and_masks_use_hardware_encodings() {
    let transmit = DataCommand::write_byte(0xa5)
        .with_restart(true)
        .with_stop(true);
    assert_eq!(transmit.bits(), 0x6a5);
    assert_eq!(transmit.data(), 0xa5);
    assert!(!transmit.read());
    assert!(transmit.restart());
    assert!(transmit.stop());
    assert_eq!(transmit.with_stop(false).with_restart(false).bits(), 0xa5);
    let read = DataCommand::read_request().with_stop(true);
    assert_eq!(read.bits(), 0x300);
    assert!(read.read());
    let receive = ReceiveData::from_bits(0x8ff);
    assert_eq!(receive.data(), 0xff);
    assert!(receive.first_data_byte());
    for (event, bit) in [
        (InterruptBits::RECEIVE_UNDERFLOW, 0),
        (InterruptBits::RECEIVE_OVERFLOW, 1),
        (InterruptBits::RECEIVE_FULL, 2),
        (InterruptBits::TRANSMIT_OVERFLOW, 3),
        (InterruptBits::TRANSMIT_EMPTY, 4),
        (InterruptBits::READ_REQUEST, 5),
        (InterruptBits::TRANSMIT_ABORT, 6),
        (InterruptBits::RECEIVE_DONE, 7),
        (InterruptBits::ACTIVITY, 8),
        (InterruptBits::STOP_DETECT, 9),
        (InterruptBits::START_DETECT, 10),
        (InterruptBits::GENERAL_CALL, 11),
        (InterruptBits::RESTART_DETECT, 12),
        (InterruptBits::MASTER_ON_HOLD, 13),
    ] {
        assert_eq!(event.bits(), 1 << bit);
        assert_eq!(
            InterruptBits::default().with_events(event, true).bits(),
            1 << bit
        );
        assert_eq!(
            InterruptBits::from_bits(u32::MAX)
                .with_events(event, false)
                .bits(),
            !(1 << bit)
        );
    }
    let mask = InterruptBits::default()
        .with_events(InterruptBits::RECEIVE_FULL, true)
        .with_events(InterruptBits::STOP_DETECT, true);
    assert_eq!(mask.bits(), 0x204);
    assert!(mask.contains(InterruptBits::from_bits(0x204)));
    assert!(!mask.contains(InterruptBits::TRANSMIT_ABORT));
    let enable = Enable::default()
        .with_enabled(true)
        .with_abort(true)
        .with_transmit_command_block(true);
    assert_eq!(enable.bits(), 7);
    assert!(enable.enabled() && enable.abort() && enable.transmit_command_block());
    assert_eq!(
        enable
            .with_enabled(false)
            .with_abort(false)
            .with_transmit_command_block(false)
            .bits(),
        0
    );
    let dma = DmaControl::default()
        .with_receive_enabled(true)
        .with_transmit_enabled(true);
    assert_eq!(dma.bits(), 3);
    assert!(dma.receive_enabled() && dma.transmit_enabled());
    let status = Status::from_bits(0x6a);
    assert!(!status.active());
    assert!(status.transmit_not_full());
    assert!(!status.transmit_empty());
    assert!(status.receive_not_empty());
    assert!(!status.receive_full());
    assert!(status.master_active() && status.slave_active());
    let status = EnableStatus::from_bits(5);
    assert!(status.enabled());
    assert!(!status.slave_disabled_while_busy());
    assert!(status.slave_receive_data_lost());
    let abort = TransmitAbortSource::from_bits((257 << 23) | 0x1_500f);
    assert_eq!(abort.flushed_entries(), 257);
    assert!(abort.user_abort() && abort.arbitration_lost() && abort.slave_arbitration_lost());
    assert!(abort.seven_bit_address_no_ack());
    assert!(abort.ten_bit_first_address_no_ack() && abort.ten_bit_second_address_no_ack());
    assert!(abort.data_no_ack());
    assert!(!abort.general_call_no_ack());
}

#[test]
fn capabilities_do_not_invent_unencoded_parameters() {
    let absent = ComponentParameters::from_bits(0x00ff_ff7f);
    assert!(!absent.encoded_parameters());
    assert_eq!(absent.transmit_fifo_depth(), None);
    assert_eq!(absent.receive_fifo_depth(), None);
    assert_eq!(absent.has_dma(), None);
    assert_eq!(absent.maximum_speed(), None);
    assert_eq!(absent.apb_data_width(), None);
    for depth in [1u16, 2, 16, 255, 256] {
        let encoded = u32::from(depth - 1);
        let params = ComponentParameters::from_bits((encoded << 16) | (encoded << 8) | 0xce);
        assert_eq!(params.transmit_fifo_depth(), Some(depth));
        assert_eq!(params.receive_fifo_depth(), Some(depth));
        assert_eq!(params.has_dma(), Some(true));
        assert_eq!(params.maximum_speed(), Some(Speed::High));
        assert_eq!(params.apb_data_width(), Some(32));
    }
    let params = ComponentParameters::from_bits(0x83);
    assert_eq!(params.maximum_speed(), None);
    assert_eq!(params.apb_data_width(), None);
    assert_eq!(params.has_dma(), Some(false));
    assert_eq!(
        ComponentParameters::from_bits(0x80).apb_data_width(),
        Some(8)
    );
    assert_eq!(
        ComponentParameters::from_bits(0x81).apb_data_width(),
        Some(16)
    );
}

struct Mock {
    words: [core::cell::UnsafeCell<u32>; 64],
}

impl Mock {
    fn new() -> Self {
        Self {
            words: core::array::from_fn(|_| core::cell::UnsafeCell::new(0)),
        }
    }

    fn registers(&self) -> &DwApbI2c {
        // SAFETY: All registers are transparent UnsafeCell-backed u32 values,
        // every bit pattern is valid, and layout tests check offsets and size.
        unsafe { &*self.words.as_ptr().cast::<DwApbI2c>() }
    }

    fn set(&self, offset: usize, value: u32) {
        // SAFETY: This test owns aligned mock memory with no concurrent users.
        unsafe { self.words[offset / 4].get().write_volatile(value) };
    }

    fn get(&self, offset: usize) -> u32 {
        // SAFETY: This test owns aligned mock memory with no concurrent users.
        unsafe { self.words[offset / 4].get().read_volatile() }
    }
}

#[test]
fn command_writes_replace_values_without_replaying_status() {
    let mock = Mock::new();
    mock.set(0x10, 0xffff_ffff);
    mock.set(0x6c, 3); // ENABLE and a previously pending ABORT.
    mock.set(0x0c, 0x1122_3344);
    mock.set(0x14, 0x5566_7788);
    let registers = mock.registers();
    // SAFETY: These registers refer only to exclusively owned ordinary test memory.
    unsafe {
        registers
            .data_command
            .write(DataCommand::write_byte(0xa5).with_stop(true));
        registers.enable.write(Enable::default().with_enabled(true));
    }
    assert_eq!(mock.get(0x10), 0x2a5);
    assert_eq!(mock.get(0x6c), 1);
    assert!(!registers.enable.read().abort());
    assert_eq!(mock.get(0x0c), 0x1122_3344);
    assert_eq!(mock.get(0x14), 0x5566_7788);
    mock.set(0x10, 0x8ab);
    // SAFETY: The mock holds a received data value and has no other users.
    let received = unsafe { registers.data_command.read() };
    assert_eq!(received.data(), 0xab);
    assert!(received.first_data_byte());
    mock.set(0x54, 1);
    // SAFETY: This reads ordinary test memory, not an actual interrupt register.
    assert_eq!(unsafe { registers.clear_transmit_abort.read() }, 1);
    // Mock memory verifies addressing only; it does not emulate FIFO pops or read-clear.
    assert_eq!(mock.get(0x54), 1);
}
