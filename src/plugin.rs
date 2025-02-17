//! `bevy` plugin definitions.
use bevy::prelude::*;

use crate::framebuffer::FrameBufferType;
use crate::schedule::SchedulePlugin;

/// Define a plugin for the given `bevy_framebuffer` backend.
macro_rules! define_plugin {
    (
        $plugin_name: ident,
        $feature: literal,
        $frame: path,
        $config: path
    ) => {
        #[cfg(feature = $feature)]
        #[doc = concat!("`bevy_framebuffer` plugin for the `", $feature, "` library. Registers [`", stringify!($frame), "`] as a `NonSend` resource.")]
        pub struct $plugin_name {
            #[doc = concat!("Configuration for the `", $feature, "` library.")]
            pub config: $config,
        }

        #[cfg(feature = $feature)]
        impl Plugin for $plugin_name {
            fn build(&self, app: &mut App) {
                type BACKEND = $frame;

                app.add_plugins(SchedulePlugin);

                let config = self.config.clone();
                app.add_systems(
                    Startup,
                    move |params: <BACKEND as FrameBufferType>::StartupParams<'_, '_>| {
                        <BACKEND as FrameBufferType>::STARTUP(params, config.clone())
                    },
                );
            }
        }
    };
}

#[cfg(feature = "pixels")]
use crate::pixels_impl;

define_plugin!(
    PixelsPlugin,
    "pixels",
    pixels_impl::PixelsFrame,
    pixels_impl::PixelsConfig
);

#[cfg(feature = "softbuffer")]
use crate::softbuffer_impl;
define_plugin!(
    SoftbufferPlugin,
    "softbuffer",
    softbuffer_impl::SoftBufferFrame,
    softbuffer_impl::SoftBufferConfig
);
