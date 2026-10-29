#![no_std]
#![no_main]

use esp_backtrace as _;
// - `analog::adc`: Tools and configurations to read continuous analog voltages
use esp_hal::{
    analog::adc::{Adc, AdcConfig, Attenuation},
    delay::Delay,
    main,
};
use esp_println as _;

mod photoresistor;
use crate::photoresistor::LightSensor;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    defmt::println!("Initializing photoresistor driver");

    // A configuration object for the Analog-to-Digital Converter
    let mut adc_config = AdcConfig::new();
    // Enables GPIO0 as an analog input pin with an 11dB attenuation
    // (whole spectrum, up to 3.3V)
    let analog_pin = adc_config.enable_pin(peripherals.GPIO0, Attenuation::_11dB);
    // Initializes the ADC1 peripheral using our configuration
    let adc1 = Adc::new(peripherals.ADC1, adc_config);

    let mut light_sensor = LightSensor::new(analog_pin, adc1);

    let delay = Delay::new();

    loop {
        if let Some(analog_val) = light_sensor.read() {
            defmt::println!("Light intensity analog value: {}", analog_val);
        }

        delay.delay_millis(1000);
    }
}
