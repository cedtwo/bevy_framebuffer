//! Framebuffer impelentation for `pixels` backend.
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::window::*;
use bevy::winit::WinitWindows;
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};

use crate::framebuffer::{FrameBuffer, FrameBufferType};

/// Framebuffer resource for `pixels` crate. Dereferences to `pixels::Pixels`.
// Note that `NonSend` resources require `'static` lifetime.
pub type PixelsFrame = FrameBuffer<Pixels<'static>>;
pub type PixelsConfig = Config;

impl FrameBufferType for PixelsFrame {
    type Config = Config;

    type StartupParams<'w, 's> = StartupParams<'w, 's>;

    const STARTUP: for<'w, 's> fn(StartupParams<'w, 's>, Self::Config) = startup;
}

/// Configuration for the `pixels` backend. Corresponds to the configuration of
/// [`pixels::builder::PixelsBuilder`].
#[derive(Clone, Debug, Default)]
pub struct Config {
    /// Framebuffer width (in pixels).
    pub width: u32,
    /// Framebuffer height (in pixels).
    pub height: u32,
    /// See [`pixels::builder::PixelsBuilder::enable_vsync`].
    pub enable_vsync: Option<bool>,
    /// See [`pixels::builder::PixelsBuilder::wgpu_backend`].
    pub wgpu_backend: Option<pixels::wgpu::Backends>,
    /// See [`pixels::builder::PixelsBuilder::present_mode`].
    pub present_mode: Option<pixels::wgpu::PresentMode>,
    /// See [`pixels::builder::PixelsBuilder::texture_format`].
    pub texture_format: Option<pixels::wgpu::TextureFormat>,
    /// See [`pixels::builder::PixelsBuilder::render_texture_format`].
    pub render_texture_format: Option<pixels::wgpu::TextureFormat>,
    /// See [`pixels::builder::PixelsBuilder::surface_texture_format`].
    pub surface_texture_format: Option<pixels::wgpu::TextureFormat>,
    /// See [`pixels::builder::PixelsBuilder::blend_state`].
    pub blend_state: Option<pixels::wgpu::BlendState>,
    /// See [`pixels::builder::PixelsBuilder::clear_color`].
    pub clear_color: Option<pixels::wgpu::Color>,
}

impl Config {
    /// Create a `Pixels` instance from the given configuration.
    fn into_pixels(
        self,
        surface: SurfaceTexture<ThreadLockedRawWindowHandleWrapper>,
    ) -> Result<Pixels<'static>, pixels::Error> {
        /// Pass the given configuration variable to the `PixelsBuilder` method
        /// of the same name.
        macro_rules! delegate_method {
            ($builder:ident, $method:ident) => {
                if let Some(var) = self.$method {
                    $builder = $builder.$method(var);
                }
            };
        }

        let mut builder = PixelsBuilder::new(self.width, self.height, surface);

        delegate_method!(builder, enable_vsync);
        delegate_method!(builder, wgpu_backend);
        delegate_method!(builder, present_mode);
        delegate_method!(builder, texture_format);
        delegate_method!(builder, render_texture_format);
        delegate_method!(builder, surface_texture_format);
        delegate_method!(builder, blend_state);
        delegate_method!(builder, clear_color);

        builder.build()
    }
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
    let raw_handle = unsafe { handle.get_handle() };

    let surface = SurfaceTexture::new(
        window.inner_size().width,
        window.inner_size().height,
        raw_handle,
    );

    let pixels = config
        .into_pixels(surface)
        .expect("Failed building `pixels` framebuffer");

    commands.queue(|world: &mut World| world.insert_non_send_resource(PixelsFrame::new(pixels)));
}
