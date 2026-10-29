// 5161AS common-cathode 7-segment with decimal point display driver
// via SN74HC595N Shift Register
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
//         (330Ω–1kΩ) to the corresponding SN74HC595N Qx output.
//
//   +---------------------------------+
//   |  16  15  14  13  12  11  10  9  |
//   |  |   |   |   |   |   |   |   |  |
//   |  XXXXXXXXXXXXXXXXXXXXXXXXXXXXX  |
//   |   XXXXXXXXXXXXXXXXXXXXXXXXXXXX  |
//   |  XXXXXXXXXXXXXXXXXXXXXXXXXXXXX  |
//   |  |   |   |   |   |   |   |   |  |
//   |  1   2   3   4   5   6   7   8  |
//   +---------------------------------+
//
// SN74HC595N connections:
//   DS   (pin 14) -> ESP32 GPIO (data pin)
//   SHCP (pin 11) -> ESP32 GPIO (shift clock)
//   STCP (pin 12) -> ESP32 GPIO (latch / storage clock)
//   OE   (pin 13) -> GND (output always enabled)
//   MR   (pin 10) -> 3.3V (no master reset)
//   GND  (pin 8)  -> GND
//   VCC  (pin 16) -> 3.3V
//
// Shift register output -> 5161AS segment mapping:
//   QA-15 -> A   QB-1 -> B   QC-2 -> C   QD-3 -> D
//   QE-4  -> E   QF-5 -> F   QG-6 -> G   QH-7 -> DP
//

use esp_hal::{delay::Delay, gpio::Output};

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

pub struct Display5161AS<'a> {
    ds: Output<'a>,
    shcp: Output<'a>,
    stcp: Output<'a>,
}

impl<'a> Display5161AS<'a> {
    // Create a new display driver backed by an SN74HC595N shift register.
    //
    // - `ds`   – serial data pin (connected to DS / pin 14)
    // - `shcp` – shift clock pin (connected to SHCP / pin 11)
    // - `stcp` – latch pin       (connected to STCP / pin 12)
    pub fn new(ds: Output<'a>, shcp: Output<'a>, stcp: Output<'a>) -> Self {
        Self { ds, shcp, stcp }
    }

    // Shift a single byte into the register, MSB first
    // DP info goes first and A last, because after 8 latches DP goes
    // to QH (pin 7), and A as last lands on QA (pin 0)
    fn shift_out(&mut self, byte: u8, delay: Delay) {
        for bit in (0..8).rev() {
            // Set data line
            if byte & (1 << bit) != 0 {
                self.ds.set_high();
            } else {
                self.ds.set_low();
            }
            // Pulse the shift clock
            self.shcp.set_high();
            delay.delay_micros(1);
            self.shcp.set_low();
            delay.delay_micros(1);
        }
    }

    // Latch the shift register contents to the output pins.
    fn latch(&mut self, delay: Delay) {
        self.stcp.set_high();
        delay.delay_micros(1);
        self.stcp.set_low();
    }

    // Display a digit (0–9) on the 7-segment display.
    pub fn set_digit(&mut self, digit: u8, dp: bool, delay: Delay) {
        let dpmask = if dp { 0b10000000 } else { 0b00000000 };
        let mask = if digit <= 9 {
            DIGIT_SEGMENTS[digit as usize] | dpmask
        } else {
            0b00000000
        };
        self.shift_out(mask, delay);
        self.latch(delay);
    }
}
