//! `bevy` plugin definition.
use bevy::prelude::*;

use crate::framebuffer::FrameBufferType;
use crate::schedule::SchedulePlugin;

/// `FrameBuffer` plugin. Registers a `FrameBufferType` as a `NonSend` resource.
pub struct FrameBufferPlugin {
    #[cfg(feature = "pixels")]
    pub config: crate::pixels_impl::Config,
    #[cfg(feature = "softbuffer")]
    pub config: crate::softbuffer_impl::Config,
}

impl Plugin for FrameBufferPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(all(feature = "pixels", feature = "softbuffer"))]
        compile_error!("Only one of the `pixels` or `softbuffer` feature can be enabled");

        #[cfg(not(any(feature = "pixels", feature = "softbuffer")))]
        compile_error!("Either the `pixels` or `softbuffer` feature must be enabled");

        #[cfg(feature = "pixels")]
        type BACKEND = crate::pixels_impl::PixelsFrame;
        #[cfg(feature = "softbuffer")]
        type BACKEND = crate::softbuffer_impl::SoftBufferFrame;

        let config = self.config.clone();

        app.add_plugins(SchedulePlugin);

        app.add_systems(
            Startup,
            move |params: <BACKEND as FrameBufferType>::StartupParams<'_, '_>| {
                <BACKEND as FrameBufferType>::STARTUP(params, config.clone())
            },
        );
    }
}
