#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    gpio::{Input, InputConfig, Pull},
    main,
};
use esp_println as _;

// Mod with debounced button logic
mod stateful_btn;
use crate::stateful_btn::{BtnState, StatefulBtn};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    defmt::println!("Initializing button driver");

    // Button connected to GND so High level means it is released
    // `Pull::Up` keeps it that way
    let input = Input::new(
        peripherals.GPIO23,
        InputConfig::default().with_pull(Pull::Up),
    );
    let mut sbtn = StatefulBtn::new(input);

    // We assume at start button in released
    let mut btn_last_state = BtnState::Released;

    loop {
        let btn_current_state = sbtn.get_state();
        // Action only on change of state
        if btn_current_state != btn_last_state {
            defmt::println!("{}", btn_current_state);
            btn_last_state = btn_current_state;
        }
    }
}
