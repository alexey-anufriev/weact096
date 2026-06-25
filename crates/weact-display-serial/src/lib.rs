//! Serial transport for `weact-display`.
//!
//! [`SerialTransport`] opens the device's USB CDC serial port
//! and implements `weact_display::Transport`, so the display driver
//! can send protocol bytes through it.

#![deny(warnings)]

use std::io::{self, Write};
use std::time::Duration;

use serialport::{FlowControl, SerialPort};
use weact_display::{Transport, TransportError};

/// Serial-port transport for the WeAct USB CDC device.
pub struct SerialTransport {
    port: Box<dyn SerialPort>,
}

impl SerialTransport {
    /// Opens a serial port with the standard WeAct settings.
    ///
    /// Uses 115200 baud, a one-second timeout, and RTS/CTS flow control.
    pub fn open(path: &str) -> io::Result<Self> {
        Self::open_with_baud_rate(path, 115_200)
    }

    /// Opens a serial port with an explicit baud rate.
    ///
    /// Intended for experiments; normal hardware access should use [`SerialTransport::open`].
    pub fn open_with_baud_rate(path: &str, baud_rate: u32) -> io::Result<Self> {
        let port = serialport::new(path, baud_rate)
            .timeout(Duration::from_secs(1))
            .flow_control(FlowControl::Hardware)
            .open()?;
        Ok(Self { port })
    }
}

impl Transport for SerialTransport {
    /// Writes all bytes to the serial port.
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), TransportError> {
        self.port.write_all(bytes).map_err(TransportError::from)
    }

    /// Flushes the serial port.
    fn flush(&mut self) -> Result<(), TransportError> {
        self.port.flush().map_err(TransportError::from)
    }
}
