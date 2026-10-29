#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{Adc, AdcCalCurve, AdcConfig, Attenuation},
    delay::Delay,
    gpio::{Input, InputConfig, Pull},
    main, peripherals,
};
use esp_println as _;

mod joystick;
use crate::joystick::JoyStick;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    defmt::println!("Starting joystick driver...");

    let mut adc_config = AdcConfig::new();
    // Enables GPIO0 as an analog input pin
    // while applying factory calibration so measurements read in millivolts
    let x_analog_pin = adc_config.enable_pin_with_cal::<peripherals::GPIO0<'_>, AdcCalCurve<_>>(
        peripherals.GPIO0,
        Attenuation::_11dB,
    );
    let y_analog_pin = adc_config.enable_pin_with_cal::<peripherals::GPIO1<'_>, AdcCalCurve<_>>(
        peripherals.GPIO1,
        Attenuation::_11dB,
    );
    let adc = Adc::new(peripherals.ADC1, adc_config);
    let button = Input::new(
        peripherals.GPIO23,
        InputConfig::default().with_pull(Pull::Down),
    );

    let mut joystick = JoyStick::new(x_analog_pin, y_analog_pin, adc, button);

    let delay = Delay::new();

    loop {
        if let Some((x, y, z)) = joystick.read() {
            defmt::println!("Readings: {} {} {}", x, y, z);
        }

        delay.delay_millis(1000);
    }
}
