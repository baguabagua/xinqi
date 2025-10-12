use crate::{general::Board, xingxiang::{general::*, utils::*}};

pub fn evaluate(board: &XingxiangBoard) -> f32 {
    let mut res = 0.0;
    let player = board.active_player;
    for x in 0..BOARD_SIZE_I {
        for y in 0..BOARD_SIZE_J {
            if let Some(p) = board.pieces[x][y] {
                let sig = if p.color == player { 1.0 } else {-1.0 };
                let mut score = match p.role {
                    XingxiangPieceRole::Pawn => { 1.0 },
                    XingxiangPieceRole::Rook => { 1.0 },
                    XingxiangPieceRole::Knight => { 1.0 },
                    XingxiangPieceRole::Bishop => { 1.0 },
                    XingxiangPieceRole::King => { 100.0 },
                };
                let offsets = p.role.offsets();
                for offset in offsets {
                    if let Some((xp, yp)) = add_offset((x, y), offset) {
                        if let Some(q) = board.pieces[xp][yp] {
                            if q.color == p.color.flip() {
                                score += match q.role {
                                    XingxiangPieceRole::Pawn => { 1.0 },
                                    XingxiangPieceRole::Rook => { 3.0 },
                                    XingxiangPieceRole::Knight => { 3.0 },
                                    XingxiangPieceRole::Bishop => { 3.0 },
                                    XingxiangPieceRole::King => { 20.0 },
                                };
                            }
                        } else {
                            score += 0.5;
                        }
                    }
                }
                res += sig * score;
            }
        }
    }
    // 把目差估计转换为（先手方）胜率估计
    1.0 / (1.0 + (-res / TEMERATURE).exp())
}

const TEMERATURE: f32 = 20.0;

pub fn quick_move(board: &XingxiangBoard) -> Vec<XingxiangStep> {
    let all_move = board.all_move();
    let kp = find_king_pos(&board.pieces, board.active_player);
    let mut make_king = false;
    if kp.is_none() && board.fullmove == 8 {
        make_king = true;
    }
    if let Some(kp) = kp {
        if can_eat_king(&board.pieces, kp, board.active_player.flip()) {
            make_king = true;
        }
    }

    if make_king {
        let mut res = Vec::new();
        for step in all_move {
            if step.change.is_some_and(|(_, p)| p.role == XingxiangPieceRole::King) {
                res.push(step);
            }
        }
        if res.is_empty() {
            return board.all_move();
        }
        return res;
    } else {
        let mut step_results: Vec<(XingxiangStep, f32)> = Vec::new();
        for step in all_move {
            let XingxiangStep { pos, change } = step;
            if board.pieces[pos.0][pos.1].is_some() {
                step_results.push((step, -10.0));
                continue;
            }
            match change {
                Some(((x, y), p)) => {
                    let mut score = 0.0;
                    if let Some(op) = board.pieces[x][y] {
                        score -= match op.role {
                            XingxiangPieceRole::Pawn => { 0.0 },
                            XingxiangPieceRole::Rook => { 3.0 },
                            XingxiangPieceRole::Knight => { 3.0 },
                            XingxiangPieceRole::Bishop => { 3.0 },
                            XingxiangPieceRole::King => { 100.0 },
                        }
                    }
                    for offset in p.role.offsets() {
                        if let Some((xp, yp)) = add_offset((x, y), offset) {
                            if let Some(q) = board.pieces[xp][yp] {
                                if q.color == p.color.flip() {
                                    score += match q.role {
                                        XingxiangPieceRole::Pawn => { 1.0 },
                                        XingxiangPieceRole::Rook => { 3.0 },
                                        XingxiangPieceRole::Knight => { 3.0 },
                                        XingxiangPieceRole::Bishop => { 3.0 },
                                        XingxiangPieceRole::King => { 20.0 },
                                    };
                                }
                            } else {
                                score += 0.5;
                            }
                        }
                    }
                    step_results.push((step, score));
                },
                None => { step_results.push((step, 1.0)) },
            }
        }
        let max_score = step_results
            .iter()
            .map(|(_, score)| score)
            .max_by(|a, b| a.total_cmp(b))
            .copied()
            .unwrap();
        let res: Vec<XingxiangStep> = step_results
            .into_iter()
            .filter(|(_, score)| *score == max_score)
            .map(|(step, _)| step)
            .collect();
        if res.is_empty() {
            return board.all_move();
        }
        return res;
    }
}