use thiserror::Error;

/// Carries screen data from the driver to the display.
pub trait Transport {
    /// Writes the full byte slice to the device.
    ///
    /// Contract: either all bytes are accepted, or an error is returned.
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), TransportError>;

    /// Flushes buffered bytes.
    ///
    /// The driver calls this after complete commands or uploads.
    /// Serial, USB, or wrapped `BufWriter` transports may keep bytes in memory temporarily;
    /// this should push any pending bytes to the device.
    fn flush(&mut self) -> Result<(), TransportError>;
}

/// Error returned by [`Transport`] implementations.
///
/// It stores a message instead of exposing transport-specific error types
/// in the core driver's API.
#[derive(Debug, Error)]
#[error("transport error: {message}")]
pub struct TransportError {
    message: String,
}

impl TransportError {
    /// Creates a transport error from a message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl From<std::io::Error> for TransportError {
    /// Converts standard I/O errors into transport errors.
    fn from(value: std::io::Error) -> Self {
        Self::new(value.to_string())
    }
}
