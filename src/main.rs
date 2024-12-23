use std::num::NonZero;

use bevy::a11y::AccessibilityPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowResolution};
use bevy::winit::{WakeUp, WinitPlugin};
use bevy_framebuffer::pixels_impl::Config;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        // DefaultPlugins.set(WindowPlugin {
        //     primary_window: Some(Window {
        //         resolution: WindowResolution::new(500.0, 400.0),
        //         // resize_constraints: WindowResizeConstraints {
        //         //     min_width: 1000.0,
        //         //     min_height: 1000.0,
        //         //     ..Default::default()
        //         // },
        //         ..Default::default()
        //     }),
        //     ..Default::default()
        // }),
        // FrameTimeDiagnosticsPlugin,
        // LogDiagnosticsPlugin::default(),
        MinimalPlugins,
        // DefaultPlugins,
        WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(500.0, 400.0),
                // resize_constraints: WindowResizeConstraints {
                //     min_width: 1000.0,
                //     min_height: 1000.0,
                //     ..Default::default()
                // },
                ..Default::default()
            }),
            ..Default::default()
        },
        AccessibilityPlugin,
        WinitPlugin::<WakeUp>::default(),
        bevy_framebuffer::FrameBufferPlugin {
            config: Config {
                // width: NonZero::new(100).unwrap(),
                // height: NonZero::new(100).unwrap(),
                width: 100,
                height: 100,
                ..Default::default()
            },
        },
    ));

    app.run();
}
