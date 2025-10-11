use bevy::prelude::*;

use crate::{graphics::XinqiGraphicsPlugin, hequn::HequnPlugin, ui::UiPlugin, xingxiang::XingxiangPlugin, zhandi::ZhandiPlugin};

mod general;
mod tree;
mod ui;
mod hequn;
mod graphics;
mod zhandi;
mod xingxiang;
mod ai;

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
        .add_plugins(ZhandiPlugin)
        .add_plugins(XingxiangPlugin)
        .add_plugins(UiPlugin)
        .run();
}

