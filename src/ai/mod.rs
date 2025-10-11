use crate::general::*;

pub mod mcts;
// pub mod hequn;
// pub mod zhandi;
// pub mod xingxiang;

pub trait AI {
    type B: Board;

    fn play(board: Self::B, time_limit_ms: u32) -> <Self::B as Board>::S;
}