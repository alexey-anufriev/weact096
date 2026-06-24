//! Driver building blocks for the WeAct Display 0.96 Inch.
//!
//! Protocol details are taken from the official
//! [`WeActStudio.SystemMonitor`](https://github.com/WeActStudio/WeActStudio.SystemMonitor)
//! repository, especially `library/lcd/lcd_comm_weact_b.py` at commit
//! 2420db509aa4dd5b205147806243f2e002bc2f33.
//!
//! The crate is organized around three pieces:
//!
//! - [`framebuffer`] owns pixels and simple drawing operations.
//! - [`protocol`] turns typed values into WeAct command bytes.
//! - [`transport`] abstracts the byte stream used by hardware.
//!
//! [`WeActDisplay`] ties those pieces together.

#![deny(warnings)]

/// RGB565 colors.
pub mod color;
/// Display driver and orientation handling.
pub mod display;
/// Operational errors.
pub mod error;
/// Framebuffer and drawing helpers.
pub mod framebuffer;
/// Protocol command encoders.
pub mod protocol;
/// Byte transport abstraction.
pub mod transport;

pub use color::Rgb565;
pub use display::WeActDisplay;
pub use error::Error;
pub use framebuffer::Framebuffer;
pub use protocol::Orientation;
pub use transport::{Transport, TransportError};

pub type Result<T> = std::result::Result<T, Error>;
