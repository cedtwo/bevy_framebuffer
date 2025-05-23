//! # Bevy FrameBuffer
//! Bevy framebuffer rendering using `pixels` or `softbuffer`.
//!
//! ## Functionality
//!
//! `bevy` framebuffer rendering with the choice of either a `pixels` or `softbuffer`
//!  pixel buffer backend. Just specify the backend as a feature, and access it with
//! `NonSend` or `NonSendMut`.
//!
//! ### Example
//!
//! ```toml
//! bevy = { version = "0.16.0", default-features = false }
//! # Enable the `pixels` or `softbuffer` feature.
//! bevy_framebuffer = { version = "0.3.0", features = ["pixels"] }
//! ```
//!
//! ```rust
//! # #[cfg(all(feature = "pixels", feature = "schedule"))]
//! # {
//! # use bevy::prelude::*;
//! # use bevy_framebuffer::FrameBufferPlugin;
//! # use bevy_framebuffer::pixels_impl::*;
//! # use bevy_framebuffer::schedule::RenderSchedule;
//! let mut app = App::new();
//! // Add `DefaultPlugins` and either `PixelsPlugin` or `SoftbufferPlugin` to your project.
//! app.add_plugins(
//!     DefaultPlugins,
//!     PixelsPlugin {
//!         config: PixelsConfig {
//!             width: 320,
//!             height: 180,
//!             ..Default::default()
//!         },
//! })
//! // Add a render system.
//! .add_systems(RenderSchedule, render_system);
//!
//! // Access the `PixelsFrame` or `SoftBufferFrame` in systems with `NonSend` or `NonSendMut`.
//! pub fn render_system(buffer: NonSendMut<PixelsFrame>) {
//!     buffer.render().unwrap();
//! }
//! # }
//! ```
//!
//! This crate, by design, only adds `FrameBuffer<T>` (and the `PixelsFrame` and/or
//! `SoftbufferFrame` aliases) as a resource and avoids adding any systems. This
//! choice was made to highlight the divergent behaviour of both libraries
//! (especially in relation to scaling/resizing) while also allowing the user a
//! degree of flexibility in how events are handled, See [`examples`] for how one
//! might implement basic systems.
//!
//! ## Backends
//!
//! This crate offers `pixels` and `softbuffer` as a framebuffer backend. **Neither**
//! plugin is enabled by default and must be enabled explicitly. Note that the
//! functionality of backends varies, and it is recommended to become familiar with
//! your backend of choice.
//!
//! Feature | Description | Exposed Type
//! ---|---|---
//! `pixels` | Adds the [`pixels`] buffer as a backend. | [`pixels::Pixels`]
//! `softbuffer` | Adds the [`softbuffer`] buffer as a backend. | [`softbuffer::Surface`]
//!
//! ## Schedules
//!
//! Two schedules are provided by `bevy_framebuffer`, `SurfaceSchedule` and `RenderSchedule`.
//! Resizing/scaling operations should be run on the `SurfaceSchedule`, which runs *after*
//! `PreUpdate`.  Rendering should be run on the `RenderSchedule` which runs *before* `Last`.
//! These schedules are included with the default `schedule` feature, which can be
//! disabled if needed.
//!
//! ## Examples
//!
//! Minimal examples are provided for both `softbuffer` and `pixels`, showing how one might
//! approach scaling and rendering for the given backend. Note that resizing is handled
//! differently for each example.
//!
//! ```bash
//! # Run the `pixels` example.
//! cargo run --example minimal_pixels --features="pixels"
//! # Run the `softbuffer` example.
//! cargo run --example minimal_softbuffer --features="softbuffer"
//! ```
//!
//! ## Safety
//!
//! This crate uses `unsafe` to expose `raw_window_handler` implementations with
//! the caveat that certain platforms do not support usage off of the main thread.
//! As such, `bevy_framebuffer` enforces main thread access on **all** platforms,
//! enforcing `FrameBuffer` as a `NonSend`/`NonSendMut` resource.
//!
//! ## Bevy compatability
//!
//! `bevy` | `pixels` | `softbuffer` | `bevy_framebuffer`
//! ---|---|---|---
//! 0.16 | 0.15 | 0.4 | 0.3
//! 0.15 | 0.15 | 0.4 | 0.2
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
