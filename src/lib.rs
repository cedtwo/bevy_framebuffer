#![doc = include_str!("../README.md")]
pub mod framebuffer;
pub mod plugin;

pub use framebuffer::FrameBuffer;
pub use plugin::*;

#[cfg(feature = "schedule")]
pub mod schedule;

#[cfg(feature = "pixels")]
pub use pixels;
#[cfg(feature = "pixels")]
pub mod pixels_impl;

#[cfg(feature = "softbuffer")]
pub use softbuffer;
#[cfg(feature = "softbuffer")]
pub mod softbuffer_impl;
