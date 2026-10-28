#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, InputConfig, Pull},
    main,
};
use esp_println as _;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    defmt::println!("Initializing movement sensor");

    // Configure GPIO pin 23 as a digital input
    // `Pull::Down` keeps the pin safely at 0V (Low) when the sensor isn't triggering it,
    // which prevents random electrical noise from causing false alarms.
    let movement_sensor = Input::new(
        peripherals.GPIO23,
        InputConfig::default().with_pull(Pull::Down),
    );

    let delay = Delay::new();

    // Delay gives sensors time to warm up or stabilize right after turning on.
    delay.delay_millis(8000);

    loop {
        // Check if GPIO pin 23 is receiving a high voltage signal
        if movement_sensor.is_high() {
            defmt::println!("Movement detected");
        }
        delay.delay_millis(1000);
    }
}
