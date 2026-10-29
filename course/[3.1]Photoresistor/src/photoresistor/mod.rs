// Simple light sensor driver built with a resistor and a photoresistor
//
// Wiring: connect 1 side of photoresistor to 3.3V and second to resistor.
//         connect resistor to GND
//         connect photoresistor - resistor connection to ADC GPIO

// - `Blocking`: One of possible peripherials reading modes
use esp_hal::{
    analog::adc::{Adc, AdcChannel, AdcPin, RegisterAccess},
    Blocking,
};

// It uses generics (`PIN`, `ADCX`) so it can work flexibly with different
// hardware pins and ADC peripherals across the ESP chip.
// - `PIN`: The specific GPIO pin connected to the sensor.
// - `ADCX`: The specific ADC peripheral unit being used.
pub struct LightSensor<'a, PIN, ADCX> {
    analog_pin: AdcPin<PIN, ADCX>,
    adc: Adc<'a, ADCX, Blocking>,
}

// The `where` clause restricts the generic types to ensure `PIN` is a valid ADC channel
// and `ADCX` provides proper hardware register access.
impl<'a, PIN, ADCX> LightSensor<'a, PIN, ADCX>
where
    PIN: AdcChannel,
    ADCX: RegisterAccess + 'a,
{
    pub fn new(analog_pin: AdcPin<PIN, ADCX>, adc: Adc<'a, ADCX, Blocking>) -> Self {
        Self { analog_pin, adc }
    }

    pub fn read(&mut self) -> Option<u16> {
        // Microcontroller hardware operations can sometimes take a moment to process.
        // We use a loop to retry until the read completes.
        loop {
            match self.adc.read_oneshot(&mut self.analog_pin) {
                Ok(value) => return Some(value),
                // If the hardware is busy/not ready yet (`WouldBlock`), ignore it and retry
                Err(nb::Error::WouldBlock) => {}
                Err(_) => return None,
            }
        }
    }
}
