// Do not use the standard library (std)
#![no_std]
// Skip the standard OS entry point (like a desktop application's main function)
#![no_main]

// Impotrts an automated panic handler built for ESP chips.
use esp_backtrace as _;
// Imports tools from `esp_hal` (Hardware Abstraction Layer):
// - `Delay`: A utility to pause code execution using hardware timers.
// - `main`: A macro that configures our entry function to run immediately after boot.
use esp_hal::{delay::Delay, main};
// Imports a logging/printing utility optimized for ESP chips
use esp_println as _;

// This macro embeds essential application metadata (such as version and app type)
// so the ESP's built-in bootloader recognizes this binary as a valid program.
esp_bootloader_esp_idf::esp_app_desc!();

// Custom attribute macro from esp_hal that marks the entry point
// The `-> !` return type means this function is designed to run forever
#[main]
fn main() -> ! {
    // Initializes the chip's internal peripherals, clocks, and power configurations
    esp_hal::init(esp_hal::Config::default());

    // Creates a new delay provider using the chip's hardware timers.
    let delay = Delay::new();
	
    // Unlike a normal desktop script that finishes and exits, embedded systems run 
	// forever.
    loop {
        // Prints "Hello world!" to the serial monitor
        defmt::println!("Hello world!");
        // Pauses the microcontroller's execution for 1 second
        delay.delay_millis(1000);
    }
}
