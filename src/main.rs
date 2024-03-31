#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello World!");
    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let mut display = Display::new(board.display_pins);

        #[allow(non_snake_case)]
        let number_0 = [
            [0, 1, 1, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
        ];

        let number_1 = [
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
        ];

        let number_2 = [
            [0, 1, 1, 1, 0],
            [0, 0, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        let number_3 = [
            [0, 1, 1, 1, 0],
            [0, 0, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 1, 0],
            [0, 1, 1, 1, 0],
        ];

        let number_4 = [
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 1, 0],
            [0, 0, 0, 1, 0],
        ];

        let number_5 = [
            [0, 1, 1, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 1, 0],
            [0, 1, 1, 1, 0],
        ];

        let number_6 = [
            [0, 1, 1, 1, 0],
            [0, 1, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
        ];

        let number_7 = [
            [0, 1, 1, 1, 0],
            [0, 0, 0, 1, 0],
            [0, 0, 0, 1, 0],
            [0, 0, 0, 1, 0],
            [0, 0, 0, 1, 0],
        ];

        let number_8 = [
            [0, 1, 1, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
        ];

        let number_9 = [
            [0, 1, 1, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 1, 0],
            [0, 0, 0, 1, 0],
        ];

        loop {
            display.show(&mut timer, number_0, 1000);
            display.show(&mut timer, number_1, 1000);
            display.show(&mut timer, number_2, 1000);
            display.show(&mut timer, number_3, 1000);
            display.show(&mut timer, number_4, 1000);
            display.show(&mut timer, number_5, 1000);
            display.show(&mut timer, number_6, 1000);
            display.show(&mut timer, number_7, 1000);
            display.show(&mut timer, number_8, 1000);
            display.show(&mut timer, number_9, 1000);
            display.clear();
            timer.delay_ms(250_u32);
        }
    }

    panic!("End");
}
