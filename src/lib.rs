//! # Bevy FrameBuffer
//! Bevy framebuffer rendering using `pixels` or `softbuffer`.
//!
//! ## Functionality
//!
//! `bevy` framebuffer rendering with the choice of either a `pixels` or `softbuffer`
//!  pixel buffer backend. Just specify the backend as a feature, and access it with
//! `NonSend` or `NonSendMut`.
//!
//! ```
//! # #[cfg(all(feature = "pixels", feature = "schedule"))]
//! # {
//! # use bevy::prelude::*;
//! # use bevy::a11y::AccessibilityPlugin;
//! # use bevy::prelude::MinimalPlugins;
//! # use bevy::window::WindowPlugin;
//! # use bevy::winit::{WakeUp, WinitPlugin};
//! # use bevy_framebuffer::FrameBufferPlugin;
//! # use bevy_framebuffer::pixels_impl::*;
//! # use bevy_framebuffer::schedule::RenderSchedule;
//! # let mut app = App::new();
//! // Add either the plugins below, or `DefaultPlugins`.
//! app.add_plugins((
//!     MinimalPlugins,
//!     WindowPlugin::default(),
//!     AccessibilityPlugin,
//!     WinitPlugin::<WakeUp>::default(),
//! ));
//! // Configure `FrameBufferPlugin` with either `PixelsConfig` or `SoftBufferConfig`,
//! // depending on the enabled feature.
//! app.add_plugins(FrameBufferPlugin {
//!     config: PixelsConfig {
//!         width: 320,
//!         height: 180,
//!         ..Default::default()
//!     },
//! })
//! // Add a render system.
//! .add_systems(RenderSchedule, render_system);
//!
//! // Access the framebuffer in systems with `NonSend` or `NonSendMut`.
//! pub fn render_system(buffer: NonSendMut<PixelsFrame>) {
//!     buffer.render().unwrap();
//! }
//! # }
//! ```
//!
//! This crate, by design, only adds `FrameBuffer<T>` as a resource and avoids
//! adding systems (including a render system) beyond that. This choice was made
//! to leave allow the user a degree of flexibility in how rendering and window
//! events are handled by the user, especially in relation to the divergent default
//! behaviour of `pixels` and `softbuffer`. See [`examples`] for how one might
//! implement basic functionality.
//!
//! ## Backends
//!
//! This crate offers `pixels` and `softbuffer` as a framebuffer backend. Note that
//! the functionality of backends varies, and it is recommended to become familiar
//! with your backend of choice. `bevy_framebuffer` requires exactly **one** of the
//! following features to be enabled:
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
//! approach scaling and rendering for the given backend.
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
//! `bevy` | `bevy_framebuffer`
//! ---|---
//! 0.15 | 0.1.0
mod framebuffer;
#[cfg(any(feature = "pixels", feature = "softbuffer"))]
mod plugin;

pub use framebuffer::FrameBuffer;

#[cfg(any(feature = "pixels", feature = "softbuffer"))]
pub use plugin::FrameBufferPlugin;

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
