#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
};
use esp_println as _;

mod digit_5161as_shift;
use crate::digit_5161as_shift::Display5161AS;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    defmt::println!("Initializing shift registry digit display driver");

    let cfg = OutputConfig::default();
    let mut display = Display5161AS::new(
        Output::new(peripherals.GPIO23, Level::Low, cfg), // DS
        Output::new(peripherals.GPIO22, Level::Low, cfg), // SHCP
        Output::new(peripherals.GPIO21, Level::Low, cfg), // STCP
    );

    let mut stream = DigitStream::new();
    let delay = Delay::new();

    loop {
        let digit = stream.next();
        defmt::println!("Displaying digit: {}", digit);

        display.set_digit(digit, false, delay);
        delay.delay_millis(1000);
        display.set_digit(digit, true, delay);
        delay.delay_millis(1000);
    }
}

struct DigitStream {
    number: u8,
}

impl DigitStream {
    fn new() -> Self {
        DigitStream { number: 0 }
    }

    fn next(&mut self) -> u8 {
        self.number = match self.number {
            9 => 0,
            i => i + 1,
        };
        self.number
    }
}
