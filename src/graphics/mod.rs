// 棋类游戏需要的绘制相关功能
use bevy::{input::common_conditions::{input_just_pressed, input_just_released}, prelude::*};
use crate::graphics::interaction::*;

pub mod entity;
pub mod interaction;
pub struct XinqiGraphicsPlugin;

impl Plugin for XinqiGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorWorldPos(None));
        app.add_event::<DragEvent>();
        app.add_event::<ClickEvent>();
        app.add_systems(Startup, setup);
        app.add_systems(
            PreUpdate,
            (
                get_cursor_world_pos,
                just_click.run_if(input_just_pressed(MouseButton::Left)),
                just_release.run_if(input_just_released(MouseButton::Left)),
                drag.run_if(resource_exists::<DragOperation>),
            ).chain(),
        );
    }
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
) {
    commands.spawn((Camera2d::default(), MainCamera));
}