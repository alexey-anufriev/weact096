/// A pixel stored in RGB565 format.
///
/// RGB565 uses 16 bits per pixel:
///
/// - red: 5 bits
/// - green: 6 bits
/// - blue: 5 bits
///
/// The display protocol sends these values little-endian;
/// use [`Rgb565::to_le_bytes`] when writing pixels to the device.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rgb565(pub u16);

impl Rgb565 {
    /// Black (`0x0000`).
    pub const BLACK: Self = Self(0x0000);

    /// White (`0xffff`).
    pub const WHITE: Self = Self(0xffff);

    /// Red (`0xf800`).
    pub const RED: Self = Self(0xf800);

    /// Green (`0x07e0`).
    pub const GREEN: Self = Self(0x07e0);

    /// Blue (`0x001f`).
    pub const BLUE: Self = Self(0x001f);

    /// Packs an 8-bit-per-channel RGB color into RGB565.
    ///
    /// The low bits are discarded, since RGB565 has fewer bits per channel
    /// than RGB888.
    pub const fn from_rgb888(r: u8, g: u8, b: u8) -> Self {
        let r = (r as u16) >> 3;
        let g = (g as u16) >> 2;
        let b = (b as u16) >> 3;
        Self((r << 11) | (g << 5) | b)
    }

    /// Returns this pixel as the two bytes expected by the display.
    ///
    /// For example, `Rgb565::RED` is `0xf800`, which becomes `[0x00, 0xf8]`.
    pub const fn to_le_bytes(self) -> [u8; 2] {
        self.0.to_le_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::Rgb565;

    #[test]
    fn converts_rgb888_to_rgb565() {
        assert_eq!(Rgb565::from_rgb888(255, 0, 0), Rgb565::RED);
        assert_eq!(Rgb565::from_rgb888(0, 255, 0), Rgb565::GREEN);
        assert_eq!(Rgb565::from_rgb888(0, 0, 255), Rgb565::BLUE);
        assert_eq!(Rgb565::from_rgb888(255, 255, 255), Rgb565::WHITE);
        assert_eq!(Rgb565::from_rgb888(0, 0, 0), Rgb565::BLACK);
    }

    #[test]
    fn serializes_as_little_endian_rgb565() {
        assert_eq!(Rgb565::RED.to_le_bytes(), [0x00, 0xf8]);
        assert_eq!(Rgb565::GREEN.to_le_bytes(), [0xe0, 0x07]);
        assert_eq!(Rgb565::BLUE.to_le_bytes(), [0x1f, 0x00]);
    }
}
