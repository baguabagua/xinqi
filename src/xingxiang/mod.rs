use bevy::prelude::*;
use crate::{general::UpdateBoard, graphics::XinqiGraphicsPlugin, tree::game_tree_event::{DeleteVariationEvent, MoveToNodeEvent}};
use crate::xingxiang::{general::*, game::*, draw::*};

pub mod general;
pub mod game;
mod utils;
mod draw;

pub struct XingxiangPlugin;

impl Plugin for XingxiangPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<XinqiGraphicsPlugin>() {
            app.add_plugins(XinqiGraphicsPlugin);
        }
        app.add_event::<EndXingxiangGame>();
        app.add_event::<UpdateBoard<XingxiangBoard>>();
        app.add_systems(Startup, xingxiang_setup);
        app.add_systems(Update, (handle_end_xingxiang_game, xingxiang_update).chain());
        app.add_systems(Update, handle_xingxiang_tree_events);
    }
}

fn handle_xingxiang_tree_events(
    mut q_xingxiang: Query<&mut XingxiangGame>,
    mut ew_update_board: EventWriter<UpdateBoard<XingxiangBoard>>,
    mut er_move_to_node: EventReader<MoveToNodeEvent>,
    mut er_delete_variation: EventReader<DeleteVariationEvent>,
) {
    let Ok(mut xingxiang) = q_xingxiang.single_mut() else {
        return;
    };
    for event in er_move_to_node.read() {
        xingxiang.tree.handle_move_to_node(event, &mut ew_update_board);
    }
    for event in er_delete_variation.read() {
        xingxiang.tree.handle_delete_variation(event, &mut ew_update_board);
    }
}