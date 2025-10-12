use crate::general::*;

pub mod mcts;
pub mod mctsv2;

pub trait AI {
    type B: Board;

    fn play(&self, board: Self::B, time_limit_ms: u32) -> <Self::B as Board>::S;
}