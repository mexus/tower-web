//! Utility types and traits.

pub mod buf_stream;
pub mod http;

#[doc(hidden)]
pub mod tuple;

mod chain;
mod connection;
mod never;
mod sealed;

pub use self::buf_stream::BufStream;
pub use self::chain::Chain;
pub use self::connection::Connection;

pub(crate) use self::never::Never;
pub(crate) use self::sealed::Sealed;
