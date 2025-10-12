use bevy::prelude::*;
use crate::{general::UpdateBoard, graphics::XinqiGraphicsPlugin, tree::game_tree_event::{DeleteVariationEvent, MoveToNodeEvent}};
use crate::zhandi::{general::*, game::*, draw::*};

pub mod general;
pub mod game;
pub mod ai;
mod draw;
mod utils;

pub struct ZhandiPlugin;

impl Plugin for ZhandiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<XinqiGraphicsPlugin>() {
            app.add_plugins(XinqiGraphicsPlugin);
        }
        app.add_event::<EndZhandiGame>();
        app.add_event::<UpdateBoard<ZhandiBoard>>();
        app.add_systems(Startup, zhandi_setup);
        app.add_systems(Update, (handle_end_zhandi_game, zhandi_update).chain());
        app.add_systems(Update, handle_zhandi_tree_events);
    }
}

fn handle_zhandi_tree_events(
    mut q_zhandi: Query<&mut ZhandiGame>,
    mut ew_update_board: EventWriter<UpdateBoard<ZhandiBoard>>,
    mut er_move_to_node: EventReader<MoveToNodeEvent>,
    mut er_delete_variation: EventReader<DeleteVariationEvent>,
) {
    let Ok(mut zhandi) = q_zhandi.single_mut() else {
        return;
    };
    for event in er_move_to_node.read() {
        zhandi.tree.handle_move_to_node(event, &mut ew_update_board);
    }
    for event in er_delete_variation.read() {
        zhandi.tree.handle_delete_variation(event, &mut ew_update_board);
    }
}