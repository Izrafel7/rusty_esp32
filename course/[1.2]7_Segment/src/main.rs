#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
};
use esp_println as _;

// A mod with display implementation
mod digit_5161as;
use crate::digit_5161as::Display5161AS;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
	
    defmt::println!("Initializing digit display driver");
	
    let cfg = OutputConfig::default();
	
    let mut display = Display5161AS::new([
        Output::new(peripherals.GPIO20, Level::Low, cfg), // A
        Output::new(peripherals.GPIO19, Level::Low, cfg), // B
        Output::new(peripherals.GPIO2,  Level::Low, cfg), // C
        Output::new(peripherals.GPIO1,  Level::Low, cfg), // D
        Output::new(peripherals.GPIO0,  Level::Low, cfg), // E
        Output::new(peripherals.GPIO22, Level::Low, cfg), // F
        Output::new(peripherals.GPIO23, Level::Low, cfg), // G
        Output::new(peripherals.GPIO3,  Level::Low, cfg), // DP
    ]);
	
	// A little helper that gives me numbers to display
    let mut stream = DigitStream::new();
    let delay = Delay::new();

    loop {
        let digit = stream.next();
        defmt::println!("Displaying digit: {}", digit);
		
        display.set_digit(digit, false);
        delay.delay_millis(1000);
        display.set_digit(digit, true);
        delay.delay_millis(1000);
    }
}

// A custom 0-9 and repeat digit cycle
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
