/// 定义了所有棋通用的接口

pub mod piece;
pub mod step; 
pub mod board;
// pub mod game;

pub use piece::*;
pub use step::*;
pub use board::*;
// pub use game::*;