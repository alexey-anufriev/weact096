use crate::protocol::Orientation;
use crate::{Error, Result};
use crate::{Framebuffer, Rgb565, Transport, protocol};

/// Driver for controlling the WeAct 0.96-inch display.
pub struct WeActDisplay<T: Transport> {
    transport: T,
    width: u16,
    height: u16,
    orientation: Orientation,
}

impl<T: Transport> WeActDisplay<T> {
    /// Creates a display using the 0.96-inch dimensions for `orientation`.
    ///
    /// This is the constructor most applications should use.
    pub fn new(transport: T, orientation: Orientation) -> Self {
        let (width, height) = orientation.dimensions();
        Self {
            transport,
            width,
            height,
            orientation,
        }
    }

    /// Current logical display width in pixels.
    pub const fn width(&self) -> u16 {
        self.width
    }

    /// Current logical display height in pixels.
    pub const fn height(&self) -> u16 {
        self.height
    }

    /// Currently configured orientation.
    pub const fn orientation(&self) -> Orientation {
        self.orientation
    }

    /// Performs the current minimal initialization sequence.
    ///
    /// TODO: read firmware details here after transports support response bytes,
    ///       then enable FastLZ when the device supports it.
    pub fn init(&mut self) -> Result<()> {
        self.set_orientation(self.orientation)?;
        Ok(())
    }

    /// Sets backlight brightness as `0..=100` percent.
    ///
    /// The protocol encoder clamps larger values.
    pub fn set_brightness(&mut self, percent: u8) -> Result<()> {
        self.transport
            .write_all(&protocol::set_brightness(percent))?;
        self.transport.flush()?;
        Ok(())
    }

    /// Changes orientation and updates dimensions.
    ///
    /// The transport is flushed, so later drawing calls use the new orientation.
    pub fn set_orientation(&mut self, orientation: Orientation) -> Result<()> {
        self.orientation = orientation;
        let (width, height) = orientation.dimensions();
        self.width = width;
        self.height = height;
        self.transport
            .write_all(&protocol::set_orientation(orientation))?;
        self.transport.flush()?;
        Ok(())
    }

    /// Sends the protocol solid-fill command.
    pub fn fill(&mut self, color: Rgb565) -> Result<()> {
        self.fill_rect(0, 0, self.width, self.height, color)
    }

    /// Fills a rectangular area with one color.
    ///
    /// The rectangle must fit inside the current logical display dimensions.
    pub fn fill_rect(
        &mut self,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        color: Rgb565,
    ) -> Result<()> {
        let x_end = x.checked_add(width);
        let y_end = y.checked_add(height);
        if width == 0
            || height == 0
            || x_end.is_none_or(|end| end > self.width)
            || y_end.is_none_or(|end| end > self.height)
        {
            return Err(Error::RectOutOfBounds {
                x,
                y,
                width,
                height,
                display_width: self.width,
                display_height: self.height,
            });
        }

        self.transport
            .write_all(&protocol::fill_rect(x, y, width, height, color))?;
        self.transport.flush()?;
        Ok(())
    }

    /// Uploads a full-screen framebuffer as uncompressed RGB565 data.
    ///
    /// The method sends:
    ///
    /// 1. a `CMD_SET_BITMAP` header for the full screen;
    /// 2. the framebuffer bytes in chunks of `width * 4`;
    /// 3. one final transport flush.
    ///
    /// The framebuffer must match the current logical dimensions.
    pub fn draw_framebuffer(&mut self, framebuffer: &Framebuffer) -> Result<()> {
        if framebuffer.width() != self.width || framebuffer.height() != self.height {
            return Err(Error::FramebufferSizeMismatch {
                actual_width: framebuffer.width(),
                actual_height: framebuffer.height(),
                display_width: self.width,
                display_height: self.height,
            });
        }

        self.transport
            .write_all(&protocol::set_bitmap_header(0, 0, self.width, self.height))?;

        // Larger displays may want row streaming to avoid building a full byte vector first.
        let bytes = framebuffer.as_rgb565_le_bytes();
        let chunk_size = self.width as usize * 4;
        for chunk in bytes.chunks(chunk_size) {
            self.transport.write_all(chunk)?;
        }

        self.transport.flush()?;
        Ok(())
    }

    /// Returns the underlying transport.
    pub fn transport(self) -> T {
        self.transport
    }
}

#[cfg(test)]
mod tests {
    use super::{Orientation, WeActDisplay};
    use crate::{Framebuffer, Rgb565, Transport, TransportError};

    #[derive(Default)]
    struct TestRecordingTransport {
        writes: Vec<Vec<u8>>,
        flushes: usize,
    }

    impl Transport for TestRecordingTransport {
        fn write_all(&mut self, bytes: &[u8]) -> Result<(), TransportError> {
            self.writes.push(bytes.to_vec());
            Ok(())
        }

        fn flush(&mut self) -> Result<(), TransportError> {
            self.flushes += 1;
            Ok(())
        }
    }

    #[test]
    fn writes_uncompressed_full_framebuffer_in_official_chunks() {
        let transport = TestRecordingTransport::default();
        let mut display = WeActDisplay::new(transport, Orientation::Landscape);
        let mut fb = Framebuffer::new_landscape();
        fb.clear(Rgb565::RED);

        display.draw_framebuffer(&fb).unwrap();
        let transport = display.transport();

        assert_eq!(
            transport.writes[0],
            vec![0x05, 0, 0, 0, 0, 159, 0, 79, 0, 0x0a]
        );
        assert_eq!(transport.writes.len(), 1 + 40);
        assert_eq!(transport.writes[1].len(), 160 * 4);
        assert_eq!(transport.writes[1][0..4], [0x00, 0xf8, 0x00, 0xf8]);
        assert_eq!(transport.flushes, 1);
    }

    #[test]
    fn init_writes_orientation() {
        let transport = TestRecordingTransport::default();
        let mut display = WeActDisplay::new(transport, Orientation::Landscape);
        display.init().unwrap();
        let transport = display.transport();
        assert_eq!(transport.writes, vec![vec![0x02, 0x02, 0x0a]]);
        assert_eq!(transport.flushes, 1);
    }

    #[test]
    fn fills_rectangles() {
        let transport = TestRecordingTransport::default();
        let mut display = WeActDisplay::new(transport, Orientation::Landscape);
        display.fill_rect(5, 2, 10, 10, Rgb565::BLUE).unwrap();
        let transport = display.transport();
        assert_eq!(
            transport.writes,
            vec![vec![0x04, 5, 0, 2, 0, 14, 0, 11, 0, 0x1f, 0x00, 0x0a]]
        );
        assert_eq!(transport.flushes, 1);
    }
}
