use esp_hal::{
    analog::adc::{Adc, AdcCalScheme, AdcChannel, AdcPin, RegisterAccess},
    gpio::Input,
    Blocking,
};

// CS goes for calibration schema
pub struct JoyStick<'a, PIN1, PIN2, ADCX, CS> {
    x_analog_pin: AdcPin<PIN1, ADCX, CS>,
    y_analog_pin: AdcPin<PIN2, ADCX, CS>,
    adc: Adc<'a, ADCX, Blocking>,
    press: Input<'a>,
}

impl<'a, PIN1, PIN2, ADCX, CS> JoyStick<'a, PIN1, PIN2, ADCX, CS>
where
    PIN1: AdcChannel,
    PIN2: AdcChannel,
    ADCX: RegisterAccess + 'a,
    CS: AdcCalScheme<ADCX>,
{
    pub fn new(
        x_analog_pin: AdcPin<PIN1, ADCX, CS>,
        y_analog_pin: AdcPin<PIN2, ADCX, CS>,
        adc: Adc<'a, ADCX, Blocking>,
        press: Input<'a>,
    ) -> Self {
        Self {
            x_analog_pin,
            y_analog_pin,
            adc,
            press,
        }
    }

    pub fn read(&mut self) -> Option<(u16, u16, bool)> {
        let y = loop {
            match self.adc.read_oneshot(&mut self.y_analog_pin) {
                Ok(value) => break Some(value),
                Err(nb::Error::WouldBlock) => {}
                Err(_) => return None,
            }
        };
        let x = loop {
            match self.adc.read_oneshot(&mut self.x_analog_pin) {
                Ok(value) => break Some(value),
                Err(nb::Error::WouldBlock) => {}
                Err(_) => return None,
            }
        };

        Some((x.unwrap(), y.unwrap(), self.press.is_high()))
    }
}
