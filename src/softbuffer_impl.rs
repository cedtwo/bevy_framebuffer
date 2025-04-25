//! Framebuffer impelentation for `softbuffer` backend.
use std::num::NonZeroU32;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::window::*;
use bevy::winit::WinitWindows;
use softbuffer::{Context, Surface};

use crate::framebuffer::{FrameBuffer, FrameBufferType};

/// Framebuffer resource for `softbuffer` crate. Dereferences to `softbuffer::Surface`.
pub type SoftBufferFrame =
    FrameBuffer<Surface<ThreadLockedRawWindowHandleWrapper, ThreadLockedRawWindowHandleWrapper>>;
pub type SoftBufferConfig = Config;

impl FrameBufferType for SoftBufferFrame {
    type Config = Config;

    type StartupParams<'w, 's> = StartupParams<'w, 's>;

    const STARTUP: for<'w, 's> fn(StartupParams<'w, 's>, Self::Config) = startup;
}

/// Configuration for the `softbuffer` backend.
#[derive(Clone, Debug)]
pub struct Config {
    /// Framebuffer width (in pixels).
    pub width: NonZeroU32,
    /// Framebuffer height (in pixels).
    pub height: NonZeroU32,
}

#[derive(SystemParam)]
pub struct StartupParams<'w, 's> {
    commands: Commands<'w, 's>,
    primary_window: Query<'w, 's, Entity, With<PrimaryWindow>>,
    winit_windows: NonSend<'w, WinitWindows>,
}

pub fn startup<'w, 's>(
    StartupParams {
        mut commands,
        primary_window,
        winit_windows,
    }: StartupParams<'w, 's>,
    config: Config,
) {
    let primary_window = primary_window
        .single()
        .expect("Expected PrimaryWindow entity");
    let window = winit_windows
        .get_window(primary_window)
        .expect("Expected winit window matching PrimaryWindow entity");
    let handle = RawHandleWrapper::new(window).unwrap();

    // SAFETY: `Framebuffer` is `!Send`, `!Sync` and threrefore only accessed on the
    // main thread.
    let (raw_display, raw_window) = unsafe { (handle.get_handle(), handle.get_handle()) };

    let mut surface = {
        let context = Context::new(raw_display).unwrap();
        Surface::new(&context, raw_window).unwrap()
    };

    surface.resize(config.width, config.height).unwrap();
    commands.queue(|world: &mut World| world.insert_non_send_resource(FrameBuffer::new(surface)));
}
