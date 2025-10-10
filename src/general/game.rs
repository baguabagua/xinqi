use bevy::{ecs::component::Mutable, prelude::*};

use crate::{general::*, tree::game_tree::GameTree};

pub trait Game: Component<Mutability = Mutable> {
    type B: Board;

    fn tree(&mut self) -> &mut GameTree<Self::B>;

    fn board(&self) -> &Self::B;
}