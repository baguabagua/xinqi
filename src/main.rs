use bevy::prelude::*;

use crate::{graphics::XinqiGraphicsPlugin, hequn::HequnPlugin, ui::UiPlugin};

mod general;
mod tree;
mod ui;
mod hequn;
mod graphics;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Xinqi".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(XinqiGraphicsPlugin)
        .add_plugins(HequnPlugin)
        .add_plugins(UiPlugin)
        .run();
}

