use bevy::prelude::*;
use crate::general::{Board, UpdateBoard};
use crate::tree::game_tree::*;

#[derive(Event)]
pub struct MoveToNodeEvent {
    pub node_id: usize,
}

impl MoveToNodeEvent {
    pub fn new(node_id: usize) -> Self {
        MoveToNodeEvent { node_id: node_id }
    }
}

#[derive(Event)]
pub struct DeleteVariationEvent {
    pub node_id: usize,
}

impl DeleteVariationEvent {
    pub fn new(node_id: usize) -> Self {
        DeleteVariationEvent { node_id: node_id }
    }
}

pub fn handle_tree_events<B: Board>(
    mut q_tree: Query<&mut GameTree<B>>,
    mut ew_update_board: EventWriter<UpdateBoard<B>>,
    mut er_move_to_node: EventReader<MoveToNodeEvent>,
    mut er_delete_variation: EventReader<DeleteVariationEvent>,
) {
    let Ok(mut tree) = q_tree.single_mut() else {
        return;
    };
    for event in er_move_to_node.read() {
        tree.handle_move_to_node(event, &mut ew_update_board);
    }
    for event in er_delete_variation.read() {
        tree.handle_delete_variation(event, &mut ew_update_board);
    }
}