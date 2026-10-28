#![no_std]
#![no_main]

use esp_backtrace as _;
// - `gpio`: A module implementing basic input/output pin logic
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
};
use esp_println as _;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // A struct containing all board peripherals (like pins, timers, etc.)
    let peripherals = esp_hal::init(esp_hal::Config::default());
	
    defmt::println!("Initializing blinking LED");
    // Configures physical pin GPIO23 as an Output pin and sets its initial state to 
	// Low with default pin configuration
    let mut led = Output::new(peripherals.GPIO23, Level::Low, OutputConfig::default());

    let delay = Delay::new();
    loop {
        // Toggles the LED state: on => off or off => on
        led.toggle();
        delay.delay_millis(1000);
    }
}
