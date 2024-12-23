//! `Framebuffer` wrapper and backend trait definitions.
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use bevy::ecs::system::SystemParam;

/// `Framebuffer` wrapper type. Dereferences to the backend framebuffer. Use the
/// type alias of the given backend for convenience.
///
/// This type is `!Send` and `!Sync` to ensure access with `NonSend`/`NonSendMut`
/// on the main thread.
pub struct FrameBuffer<T>(T, PhantomData<*const ()>);

impl<T> Deref for FrameBuffer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for FrameBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> FrameBuffer<T> {
    pub fn new(buffer: T) -> Self {
        Self(buffer, PhantomData)
    }
}

/// Associated configuration, systems and parameters for supported `FrameBuffer`
/// backends.
pub(crate) trait FrameBufferType: Sized {
    /// Configuration for the given `FrameBufferType`.
    type Config;

    /// `Self::startup` system parameters.
    type StartupParams<'w, 's>: SystemParam;
    /// `Framebuffer` configuration and initialization system.
    const STARTUP: for<'w, 's> fn(Self::StartupParams<'w, 's>, Self::Config);
}
