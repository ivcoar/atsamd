#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

#[cfg(feature = "panic_halt")]
pub extern crate panic_halt;

use hal::prelude::*;
use hal::*;

pub use hal::target_device as pac;
pub use hal::common::*;
pub use hal::samd21::*;

use gpio::{Floating, Input, Port};

// The docs could be further improved with details of the specific channels etc
define_pins!(
    /// Maps the pins to their arduino names and the numbers printed on the board.
    /// Information from: <https://github.com/arduino/ArduinoCore-samd/blob/master/variants/nano_33_iot/variant.cpp>
    struct Pins,
    target_device: target_device,

    /// Digital 2: PWM
    pin d2 = b19,

    /// Digital 3: PWM
    pin d3 = b11,

    /// Digital 4:
    pin d4 = a7,

    /// Digital 5: PWM
    pin d5 = a5,

    /// Digital 6: PWM
    pin d6 = a4,

    /// Digital 7:
    pin d7 = a6,

    /// Digital 8:
    pin d8 = a18,

    /// Digital 9: PWM
    pin d9 = a20,

    /// Digital 10: PWM
    pin d10 = a21,

    /// Digital 11: PWM
    pin d11 = a16,

    /// Digital 12: PWM
    pin d12 = a19,

    /// Digital 13: LED
    pin d13 = a17,

    /// Analog 0:
    pin a0 = a2,

    /// Analog 1:
    pin a1 = b2,

    /// Analog 2:
    pin a2 = a11,

    /// Analog 3:
    pin a3 = a10,

    /// Analog 4:
    pin a4 = b8,

    /// Analog 5:
    pin a5 = b9,

    /// Analog 6:
    pin a6 = a9,

    /// Analog 7:
    pin a7 = b3,

/*
    /// SPI MOSI: PWM, TCC
    pin mosi = a16,

    /// SPI SCK
    pin sck = a17,

    /// SPI MISO: PWM, TC
    pin miso = a19,

    /// SDA
    pin sda = b8,

    /// SCL
    pin scl = b9,

*/

    pin usb_n = a24,
    pin usb_p = a25,
    pin aref = a3,
);
