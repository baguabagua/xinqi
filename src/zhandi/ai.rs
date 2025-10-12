use crate::zhandi::{general::*, utils::*};

pub fn evaluate(board: &ZhandiBoard) -> f32 {
    let mut res = 0.0;
    for x in 0..BOARD_DIAMETER {
        for y in 0..BOARD_DIAMETER {
            if !valid_coordinate(x, y) {
                continue;
            }
            let mut cell_res: i32 = 0;
            for (dx, dy) in OFFSET {
                if let Some((xp, yp)) = add_offset((x, y), (dx, dy)) {
                    cell_res += match board.pieces[xp][yp] {
                        Some(p) => {
                            match p {
                                ZhandiPiece::Black => 1,
                                ZhandiPiece::White => -1,
                            }
                        },
                        None => 0,
                    }
                }
            }
            res += cell_res.signum() as f32;
        }
    }
    // 把目差估计转换为（先手方）胜率估计
    1.0 / (1.0 + (-res / TEMERATURE).exp())
}

const TEMERATURE: f32 = 10.0;

pub fn quick_move(board: &ZhandiBoard) -> Vec<ZhandiStep> {
    let mut step_results = Vec::new();
    let mut net_value: Vec<Vec<i32>> = vec![vec![0; BOARD_DIAMETER]; BOARD_DIAMETER];
    let mut score = 0;

    for x in 0..BOARD_DIAMETER {
        for y in 0..BOARD_DIAMETER {
            if !valid_coordinate(x, y) {
                continue;
            }
            for (dx, dy) in OFFSET {
                if let Some((xp, yp)) = add_offset((x, y), (dx, dy)) {
                    net_value[x][y] += match board.pieces[xp][yp] {
                        Some(p) => {
                            match p {
                                ZhandiPiece::Black => 1,
                                ZhandiPiece::White => -1,
                            }
                        },
                        None => 0,
                    }
                }
            }
            score += net_value[x][y].signum();
        }
    }

    for x in 0..BOARD_DIAMETER {
        for y in 0..BOARD_DIAMETER {
            if !valid_coordinate(x, y) || board.pieces[x][y].is_some() {
                continue;
            }
            let mut new_score = score;
            for (dx, dy) in OFFSET {
                if let Some((xp, yp)) = add_offset((x, y), (dx, dy)) {
                    match board.active_player {
                        ZhandiPiece::Black => {
                            if net_value[xp][yp] == -1 || net_value[xp][yp] == 0 {
                                new_score += 1;
                            }
                        },
                        ZhandiPiece::White => {
                            if net_value[xp][yp] == 1 || net_value[xp][yp] == 0 {
                                new_score -= 1;
                            }
                        },
                    }
                }
            }
            step_results.push((ZhandiStep::Pos(x, y), new_score));
        }
    }

    match board.active_player {
        ZhandiPiece::Black => {
            let max_score = step_results
                .iter()
                .map(|(_, score)| score)
                .max()
                .copied()
                .unwrap();
            return step_results
                .into_iter()
                .filter(|(_, score)| *score == max_score)
                .map(|(step, _)| step)
                .collect();
        },
        ZhandiPiece::White => {
            let min_score = step_results
                .iter()
                .map(|(_, score)| score)
                .min()
                .copied()
                .unwrap();
            return step_results
                .into_iter()
                .filter(|(_, score)| *score == min_score)
                .map(|(step, _)| step)
                .collect();
        },
    }
}