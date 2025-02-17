use bevy::prelude::*;
use bevy::window::WindowResized;

use bevy_framebuffer::pixels_impl::{PixelsConfig, PixelsFrame};
use bevy_framebuffer::schedule::{RenderSchedule, SurfaceSchedule};
use bevy_framebuffer::PixelsPlugin;

/// The fixed buffer width.
const BUFFER_WIDTH: u32 = 320;
/// The fixed buffer height.
const BUFFER_HEIGHT: u32 = 180;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        PixelsPlugin {
            config: PixelsConfig {
                width: BUFFER_WIDTH,
                height: BUFFER_HEIGHT,
                ..Default::default()
            },
        },
    ));

    app.add_systems(SurfaceSchedule, resize_system);
    app.add_systems(Update, draw_system);
    app.add_systems(RenderSchedule, render_system);

    app.run();
}

/// Resize the surface.
pub fn resize_system(mut events: EventReader<WindowResized>, mut buffer: NonSendMut<PixelsFrame>) {
    if let Some(event) = events.read().into_iter().last() {
        buffer
            .resize_surface(event.width as u32, event.height as u32)
            .unwrap()
    }
}

/// Draw a fixed-size box in the center of the buffer.
pub fn draw_system(mut buffer: NonSendMut<PixelsFrame>) {
    const WHITE: [u8; 4] = [255, 255, 255, 255];
    const BLUE: [u8; 4] = [0, 0, 255, 255];

    const BOX_WIDTH: usize = 50;
    const BOX_HEIGHT: usize = 50;

    const BOX_T: usize = (BUFFER_HEIGHT / 2) as usize - (BOX_WIDTH / 2) as usize;
    const BOX_L: usize = (BUFFER_WIDTH / 2) as usize - (BOX_WIDTH / 2) as usize;
    const BOX_B: usize = (BUFFER_HEIGHT / 2) as usize + (BOX_HEIGHT / 2) as usize;
    const BOX_R: usize = (BUFFER_WIDTH / 2) as usize + (BOX_HEIGHT / 2) as usize;

    buffer
        .frame_mut()
        .chunks_mut(4)
        .enumerate()
        .for_each(|(i, pixel)| {
            let x = i % BUFFER_WIDTH as usize;
            let y = i / BUFFER_WIDTH as usize;

            if (x > BOX_L && x < BOX_R) && (y > BOX_T && y < BOX_B) {
                pixel.copy_from_slice(&BLUE)
            } else {
                pixel.copy_from_slice(&WHITE)
            }
        });
}

pub fn render_system(buffer: NonSend<PixelsFrame>) {
    buffer.render().unwrap();
}
