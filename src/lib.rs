#![no_std]

pub mod angle;
pub mod debug;
pub mod event;
pub mod input;
pub mod jobs;
pub mod sprite;
pub mod time;
pub mod view;

extern crate alloc;

use bevy_app::{App, Plugin};

pub struct DefaultPlugins;

impl Plugin for DefaultPlugins {
    fn build(&self, app: &mut App) {
        playdate::println!("test");
        app.add_plugins((
            input::InputPlugin,
            sprite::SpritePlugin,
            time::TimePlugin,
            debug::DebugPlugin,
            view::ViewPlugin,
            bevy_transform::TransformPlugin,
        ));
    }
}
