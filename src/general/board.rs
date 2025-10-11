use bevy::prelude::*;

use crate::general::*;

pub trait Board: Default + Clone + Send + Sync + 'static {
    type S: Step;

    // 尝试进行一步移动，如果成功，返回移动后的棋盘
    fn try_move(&self, step: Self::S) -> Option<Self> where Self: Sized;

    // 显示当前可行的所有移动
    fn all_move(&self) -> Vec<Self::S>;

    // 查询本局游戏是否已结束
    fn end_game(&self) -> bool;

    // 显示提示信息，提示当前行动方以及应该采取的行动，或游戏结果
    fn game_info(&self) -> &str;

    // 获取当前的回合数
    fn get_fullmove(&self) -> usize;

    // 获取本局胜利方, None 表示和棋。只有当本局已结束时才有效。
    fn get_winner(&self) -> Option<PlayerOrder>;

    // 获取当前的行动方
    fn get_active_player(&self) -> PlayerOrder;

    // 在一步移动和它的字符串表示间进行转换
    fn read_step(&self, s: String) -> Option<Self::S>;
    fn write_step(&self, step: Self::S) -> Option<String>;

    // 在棋盘状态与字符串表示间进行转换
    fn read_fen(s: String) -> Option<Self> where Self: Sized;
    fn write_fen(&self) -> String;
}

pub type StepType<B> = <B as Board>::S;

#[derive(Event)]
pub struct UpdateBoard<B: Board> {
    pub new_board: B,
}

impl<B: Board> UpdateBoard<B> {
    pub fn new(board: B) -> Self {
        Self {
            new_board: board,
        }
    }
}