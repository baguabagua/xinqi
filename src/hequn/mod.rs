use bevy::prelude::*;
use crate::{general::UpdateBoard, graphics::XinqiGraphicsPlugin, hequn::{game::*, general::HequnBoard}, tree::game_tree_event::*};

pub mod general;
pub mod game;
mod utils;

pub struct HequnPlugin;

impl Plugin for HequnPlugin {
    fn build(&self, app: &mut App) {
           if !app.is_plugin_added::<XinqiGraphicsPlugin>() {
            app.add_plugins(XinqiGraphicsPlugin);
        }
        app.add_event::<EndHequnGame>();
        app.add_event::<UpdateBoard<HequnBoard>>();
        app.add_systems(Startup, hequn_setup);
        app.add_systems(Update, (handle_end_hequn_game, hequn_update).chain());
        app.add_systems(Update, handle_hequn_tree_events);
    }
}

fn handle_hequn_tree_events(
    mut q_hequn: Query<&mut HequnGame>,
    mut ew_update_board: EventWriter<UpdateBoard<HequnBoard>>,
    mut er_move_to_node: EventReader<MoveToNodeEvent>,
    mut er_delete_variation: EventReader<DeleteVariationEvent>,
) {
    let Ok(mut hequn) = q_hequn.single_mut() else {
        return;
    };
    for event in er_move_to_node.read() {
        hequn.tree.handle_move_to_node(event, &mut ew_update_board);
    }
    for event in er_delete_variation.read() {
        hequn.tree.handle_delete_variation(event, &mut ew_update_board);
    }
}