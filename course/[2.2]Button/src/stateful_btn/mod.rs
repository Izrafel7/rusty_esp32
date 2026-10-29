// Simple button driver that allows input debouncing
//
// Standard button pinout
//
//   +---------------------+
//   |   D |         | C   |
//   |     +---------+     |
//   |     |\   O   /|     |
//   |     | |O O O| |     |
//   |     |/   O   \|     |
//   |     +---------+     |
//   |   A |         | B   |
//   +---------------------+
//
// Wiring: A-D are connected by default, so are B-C
//         Connect one of pins to GND,
//         second (not connected to first one) to ESP32 GPIO

// -`time`: A module with utilities for time measurement
use esp_hal::{
    gpio::Input,
    time::{Duration, Instant},
};

// Timeframe of expected fluctuating signals
const DEBOUNCE_TIME_MS: u64 = 50;

// We know it is about buttons, so call it properly
#[derive(PartialEq, Copy, Clone, defmt::Format)]
pub enum BtnState {
    Pressed,
    Released,
}

// We use internal clocks
struct Debouncer {
    timer: Option<Instant>,
}

impl Debouncer {
    fn new() -> Self {
        Self { timer: None }
    }

    fn is_debounce_timeuot_active(&self) -> bool {
        match &self.timer {
            Some(clock) => clock.elapsed() < Duration::from_millis(DEBOUNCE_TIME_MS),
            _ => false,
        }
    }

    // Button was pressed (probably), if it counts - initialize timer
    fn click(&mut self) {
        if !self.is_debounce_timeuot_active() {
            self.timer = Some(Instant::now());
        }
    }

    // Releasing clears timer
    fn unclick(&mut self) {
        self.timer = None;
    }
}

// Button struct couples input with designated timer
// It allows filtering false signals so the system
// recognizes only one intentional press or release
pub struct StatefulBtn<'a> {
    input: Input<'a>,
    debouncer: Debouncer,
}

impl<'a> StatefulBtn<'a> {
    pub fn new(input: Input<'a>) -> Self {
        let db = Debouncer::new();
        Self {
            input: input,
            debouncer: db,
        }
    }

    // We get Low - pressed, High - released
    // After initial press, button can register a few of rapid-fire clicks,
    // so after one we activate debounce timer
    pub fn get_state(&mut self) -> BtnState {
        match (
            self.input.is_high(),
            self.debouncer.is_debounce_timeuot_active(),
        ) {
            (false, false) => {
                self.debouncer.click();
                BtnState::Pressed
            }
            (true, false) => {
                self.debouncer.unclick();
                BtnState::Released
            }
            _ => BtnState::Pressed,
        }
    }
}
