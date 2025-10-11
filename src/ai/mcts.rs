use std::{marker::PhantomData, time::{Duration, Instant}};

// Monte-Carlo Tree Search
use bevy::{prelude::*};
use crate::{ai::AI, general::*};
use rand::Rng;

struct MCTSNode<B: Board> {
    visit_count: u32,
    win_count: f32, // 先手方获胜次数，和棋计 0.5
    all_move: Vec<B::S>,
    sons: Vec<Option<usize>>,
    son_num: usize,
    unselected_son_num: usize,
}

impl<B> MCTSNode<B> 
where B: Board 
{
    fn new(board: &B) -> Self {
        let all_move = board.all_move();
        let len = all_move.len();
        Self { 
            visit_count: 0, 
            win_count: 0.0, 
            all_move,
            sons: vec![None; len],
            son_num: len,
            unselected_son_num: len,
        }
    }
}

struct MCTS<B: Board> {
    board: B,
    nodes: Vec<MCTSNode<B>>,
    root: usize,
    exploration_param: f32,
    rng: rand::rngs::ThreadRng,
}

impl<B> MCTS<B> 
where B: Board 
{
    fn new(board: B, p: f32) -> Self {
        let nodes = vec![MCTSNode::new(&board)];
        Self { 
            board,
            nodes, 
            root: 0, 
            exploration_param: p,  
            rng: rand::rng(),
        }
    }

    // 单次搜索，快速行棋策略为在所有可能行动中随机一种
    fn search(&mut self) {
        let mut board = self.board.clone();
        let mut current = self.root;
        let mut node_path = vec![current];

        // 选择阶段
        while self.nodes[current].unselected_son_num == 0 {
            // 如果走到了一个游戏已结束的节点，则直接退出选择阶段，并跳过扩展阶段和模拟阶段
            if self.nodes[current].son_num == 0 {
                break;
            }
            let index = self.select(current, board.get_active_player());
            board = board.try_move(self.nodes[current].all_move[index]).unwrap();
            current = self.nodes[current].sons[index].unwrap();
            node_path.push(current);
        }

        if self.nodes[current].son_num > 0 {
            // 扩展阶段
            let mut flag = false;
            for i in 0..self.nodes[current].son_num {
                if self.nodes[current].sons[i] == None {
                    self.nodes[current].unselected_son_num -= 1;
                    board = board.try_move(self.nodes[current].all_move[i]).unwrap();
                    self.nodes.push(MCTSNode::new(&board));
                    let new_node = self.nodes.len() - 1;
                    self.nodes[current].sons[i] = Some(new_node);
                    current = new_node;
                    node_path.push(current);
                    flag = true;
                    break;
                }
            }
            if !flag {
                error!("MCTS: Expand a node that cannot be expanded");
                return;
            }

            // 模拟阶段，每次在所有可能走法中随机一种
            while !board.end_game() {
                let all_move = board.all_move();
                let num = self.rng.random_range(0..all_move.len());
                board = board.try_move(all_move[num]).unwrap();
            }
        }

        // 回溯阶段
        let win_count = match board.get_winner() {
            Some(winner) => match winner {
                PlayerOrder::First => 1.0,
                PlayerOrder::Second => 0.0,
            },
            None => 0.5,
        };
        for node in node_path {
            self.nodes[node].visit_count += 1;
            self.nodes[node].win_count += win_count;
        }
    }

    /**
     * 从当前节点的所有儿子中选择一个本步行动，返回选择的儿子的排名。
     * 使用 UCB1 公式 score = Q + c * sqrt(ln(N) / n)
     * 其中 Q 为估测胜率, c 为参数, N 为父节点访问次数, n 为子节点访问次数。
     */ 
    fn select(&self, current: usize, active_player: PlayerOrder) -> usize {
        let son_num = self.nodes[current].son_num;
        let nn = self.nodes[current].visit_count as f32;
        let mut best_score: f32 = -10.0;
        let mut best_index: usize = son_num;
        for i in 0..son_num {
            let nd = self.nodes[current].sons[i].unwrap();
            let n = self.nodes[nd].visit_count as f32;
            let qq = match active_player {
                PlayerOrder::First => self.nodes[nd].win_count / self.nodes[nd].visit_count as f32,
                PlayerOrder::Second => 1.0 - self.nodes[nd].win_count / self.nodes[nd].visit_count as f32,
            };
            let score = qq + self.exploration_param * (nn.ln() / n).sqrt();
            if score > best_score {
                best_score = score;
                best_index = i;
            }
        }
        best_index
    }

    fn get_best_move(&self) -> B::S {
        let mut best_visit_count = 0;
        let mut best_index = 0;

        for i in 0..self.nodes[self.root].son_num {
            if let Some(node_index) = self.nodes[self.root].sons[i] {
                let visit_count = self.nodes[node_index].visit_count;
                if visit_count > best_visit_count {
                    best_visit_count = visit_count;
                    best_index = i;
                }
            }
        }

        self.nodes[self.root].all_move[best_index].clone()
    }
}

pub struct MCTSAI<B: Board> {
    _marker: PhantomData<B>,
}

impl<B> AI for MCTSAI<B>
where B: Board
{
    type B = B;

    fn play(board: Self::B, time_limit_ms: u32) -> <Self::B as Board>::S {
        let start_time = Instant::now();
        let time_limit = Duration::from_millis(time_limit_ms as u64);
        
        // 创建MCTS实例，使用常见的探索参数√2
        let mut mcts = MCTS::new(board, std::f32::consts::SQRT_2);
        
        // 在时限内尽可能多地搜索
        while start_time.elapsed() < time_limit {
            mcts.search();
        }
        
        info!("MCTS completed {} simulations", mcts.nodes[mcts.root].visit_count);
        
        // 返回访问次数最多的着法
        mcts.get_best_move()
    }
}