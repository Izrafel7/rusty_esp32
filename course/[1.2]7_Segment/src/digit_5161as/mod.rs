// 5161AS common-cathode 7-segment with decimal point display driver
//
// 5161AS pinout (front view, decimal point bottom-right):
//
//           TOP ROW
//      G    F  COM  A   B
//   +---------------------+
//   |          A          |
//   |        +---+        |
//   |      F |   | B      |
//   |        +---+        |
//   |      E | G | C      |
//   |        +---+   . DP |
//   |          D          |
//   +---------------------+
//      E    D  COM  C   DP
//          BOTTOM ROW
//
// Wiring: Connect COM pins (3 and 8) to GND.
//         Connect each segment pin through a current-limiting resistor
//         (330Ω–1kΩ) to the corresponding ESP32 GPIO.
//

use esp_hal::gpio::Output;

// Segment bitmask lookup for digits 0–9.
// `1` means the segment is ON.
const DIGIT_SEGMENTS: [u8; 10] = [
    // bit order: DP G F E D C B A
    0b00111111, // 0: A B C D E F
    0b00000110, // 1: B C
    0b01011011, // 2: A B D E G
    0b01001111, // 3: A B C D G
    0b01100110, // 4: B C F G
    0b01101101, // 5: A C D F G
    0b01111101, // 6: A C D E F G
    0b00000111, // 7: A B C
    0b01111111, // 8: A B C D E F G
    0b01101111, // 9: A B C D F G
];

// 8 pins to be stored
// - `'a`: A lifetime parameter required because the Output borrows hardware references.
pub struct Display5161AS<'a> {
    pins: [Output<'a>; 8], // A, B, C, D, E, F, G, DP
}

impl<'a> Display5161AS<'a> {
    // Pass pins in order
    pub fn new(pins: [Output<'a>; 8]) -> Self {
        Self { pins }
    }

    // Display a digit (0–9) on the 7-segment display.
    pub fn set_digit(&mut self, digit: u8, dp: bool) {
		let dpmask = if dp {
			0b10000000
		} else {
			0b00000000
		};
        let mask = if digit <= 9 {
			 DIGIT_SEGMENTS[digit as usize] | dpmask
		} else {
			0b00000000
		};
		
        for i in 0..8 {
            if mask & (1 << i) != 0 {
                self.pins[i as usize].set_high();
            } else {
                self.pins[i as usize].set_low();
            }
        }
    }
}
