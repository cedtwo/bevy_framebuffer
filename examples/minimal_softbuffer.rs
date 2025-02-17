use std::num::NonZeroU32;

use bevy::prelude::*;
use bevy::window::WindowResized;

use bevy_framebuffer::schedule::{RenderSchedule, SurfaceSchedule};
use bevy_framebuffer::softbuffer_impl::{SoftBufferConfig, SoftBufferFrame};
use bevy_framebuffer::SoftbufferPlugin;

/// The initial, and minimum buffer width.
const INITIAL_BUFFER_WIDTH: u32 = 320;
/// The initial, and minimum buffer height.
const INITIAL_BUFFER_HEIGHT: u32 = 180;

/// The current buffer `width` and `height`. Provides dimensions for rendering
/// in relation to window resizes.
#[derive(Resource)]
pub struct BufferSize {
    width: u32,
    height: u32,
}

impl Default for BufferSize {
    fn default() -> Self {
        Self {
            width: INITIAL_BUFFER_WIDTH,
            height: INITIAL_BUFFER_HEIGHT,
        }
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        SoftbufferPlugin {
            config: SoftBufferConfig {
                width: NonZeroU32::new(INITIAL_BUFFER_WIDTH).unwrap(),
                height: NonZeroU32::new(INITIAL_BUFFER_HEIGHT).unwrap(),
            },
        },
    ));

    app.init_resource::<BufferSize>();
    app.add_systems(SurfaceSchedule, resize_system);
    app.add_systems(Update, draw_system);
    app.add_systems(RenderSchedule, render_system);

    app.run();
}

/// Resize the buffer if greater than the minimum initial size. Store the new
/// buffer size in the `BufferSize` resource.
pub fn resize_system(
    mut events: EventReader<WindowResized>,
    mut buffer: NonSendMut<SoftBufferFrame>,
    mut buffer_size: ResMut<BufferSize>,
) {
    if let Some(event) = events.read().into_iter().last() {
        buffer_size.width = (event.width as u32).max(INITIAL_BUFFER_WIDTH);
        buffer_size.height = (event.height as u32).max(INITIAL_BUFFER_HEIGHT);

        buffer
            .resize(
                NonZeroU32::new(buffer_size.width).unwrap(),
                NonZeroU32::new(buffer_size.height).unwrap(),
            )
            .unwrap();
    }
}

/// Draw a fixed-size box in the center of the buffer.
pub fn draw_system(mut buffer: NonSendMut<SoftBufferFrame>, buffer_size: Res<BufferSize>) {
    const WHITE: u32 = u32::from_be_bytes([0, 255, 255, 255]);
    const BLUE: u32 = u32::from_be_bytes([0, 0, 0, 255]);

    const BOX_WIDTH: usize = 50;
    const BOX_HEIGHT: usize = 50;

    let box_t: usize = ((buffer_size.height / 2) as usize).saturating_sub(BOX_HEIGHT / 2);
    let box_l: usize = ((buffer_size.width / 2) as usize).saturating_sub(BOX_WIDTH / 2);
    let box_b: usize = (buffer_size.height / 2) as usize + (BOX_HEIGHT / 2) as usize;
    let box_r: usize = (buffer_size.width / 2) as usize + (BOX_WIDTH / 2) as usize;

    buffer
        .buffer_mut()
        .unwrap()
        .iter_mut()
        .enumerate()
        .for_each(|(i, pixel)| {
            let x = i % buffer_size.width as usize;
            let y = i / buffer_size.width as usize;

            if (x > box_l && x < box_r) && (y > box_t && y < box_b) {
                *pixel = BLUE;
            } else {
                *pixel = WHITE;
            }
        });
}

pub fn render_system(mut buffer: NonSendMut<SoftBufferFrame>) {
    buffer.buffer_mut().unwrap().present().unwrap();
}
