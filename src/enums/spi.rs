//! SPI clock and latch configuration enums.

/// SPI clock/latch behaviour modes.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiMode {
    /// First bit output immediately when CS activates. READ-CLK_UP-DELAY-CLK_DOWN-DELAY
    Mode0 = 0,
    /// First bit output on first clock edge after CS activates. CLK_UP-DELAY-READ-CLK_DOWN-DELAY
    Mode1 = 1,
    /// First bit output immediately when CS activates. READ-CLK_DOWN-DELAY-CLK_UP-DELAY
    Mode2 = 2,
    /// First bit output on first clock edge after CS activates. CLK_DOWN-DELAY-READ-CLK_UP-DELAY
    Mode3 = 3,
}

/// Polarity of the SPI chip-select (nCS) latch pulse.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiLatchMode {
    /// nCS goes UP for triggering SPI latch.
    LatchUp = 0,
    /// nCS goes DOWN for triggering SPI latch.
    LatchDown = 1,
}
