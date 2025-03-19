pub mod ffmpeg;
pub mod error;
pub mod formats;  // New module for format handling

pub use ffmpeg::*;
pub use error::*;
pub use formats::*;