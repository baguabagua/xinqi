use bevy::{ecs::query::QueryData, prelude::*};

use crate::{general::*, tree::game_tree::GameTree};

pub trait Game: Component + QueryData<Mutability = bevy::ecs::query::Mutable> {
    type B: Board;

    fn get_tree(&mut self) -> &mut GameTree<Self::B>;
}