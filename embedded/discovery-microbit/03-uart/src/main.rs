#![no_main]
#![no_std]
use core::fmt::Write;

use cortex_m_rt::entry;
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

mod serial_setup;
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    nb::block!(serial.write(b'X')).unwrap();
    write!(serial, "The quick brown fox jumps over the lazy dog.\r\n").unwrap();
    nb::block!(serial.flush()).unwrap();

    loop {}
}
