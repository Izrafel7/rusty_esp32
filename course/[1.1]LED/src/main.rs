#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
};
use esp_println as _;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    defmt::println!("Initializing blinking LED");

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let mut led = Output::new(peripherals.GPIO23, Level::Low, OutputConfig::default());

    let delay = Delay::new();
    loop {
        led.toggle();
        delay.delay_millis(1000);
    }
}
