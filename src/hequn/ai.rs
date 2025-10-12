use crate::{hequn::{general::*, utils::*}};
use std::{sync::OnceLock};

static LOCAL_RANDIAN: OnceLock<[[Vec<((usize, usize), (usize, usize))>; 5]; 5]> = OnceLock::new();

fn get_local_randian() -> &'static [[Vec<((usize, usize), (usize, usize))>; 5]; 5] {
    LOCAL_RANDIAN.get_or_init(|| {
        let mut res: [[Vec<((usize, usize), (usize, usize))>; 5]; 5] = [
            [vec![], vec![], vec![], vec![], vec![]],
            [vec![], vec![], vec![], vec![], vec![]],
            [vec![], vec![], vec![], vec![], vec![]],
            [vec![], vec![], vec![], vec![], vec![]],
            [vec![], vec![], vec![], vec![], vec![]],
        ];
        for cx in 1..=3 {
            for cy in 1..=3 {
                for (dx, dy) in [(1, 0), (0, 1), (1, 1), (1, -1)] {
                    let (x0, y0) = (cx as usize, cy as usize);
                    let (x1, y1) = ((cx + dx) as usize, (cy + dy) as usize);
                    let (x2, y2) = ((cx - dx) as usize, (cy - dy) as usize);
                    res[x0][y0].push(((x1, y1), (x2, y2)));
                    res[x1][y1].push(((x0, y0), (x2, y2)));
                    res[x2][y2].push(((x1, y1), (x0, y0)));
                }
            }
        }
        res
    })
}

#[derive(Clone, Copy, PartialEq)]
pub enum HequnPiecePlus {
    Piece(HequnPiece),
    None,
    Wall,
}

// 只计算染点数量差，判断关键区域
fn evaluate_critical(pieces: &Vec<Vec<HequnPiecePlus>>, cx: usize, cy: usize) -> bool {
    let mut black_randian: i32 = 0;
    let mut white_randian: i32 = 0;
    for dx in 0..=4 {
        for dy in 0..=4 {
            let (x, y) = (cx + dx, cy + dy);
            if pieces[x][y] == HequnPiecePlus::None {
                let mut br = false;
                let mut wr = false;
                for ((x1, y1), (x2, y2)) in get_local_randian()[dx][dy].clone() {
                    if pieces[cx + x1][cy + y1] == HequnPiecePlus::Piece(HequnPiece::Black) && pieces[cx + x2][cy + y2] == HequnPiecePlus::Piece(HequnPiece::Black) {
                        br = true;
                    }
                    if pieces[cx + x1][cy + y1] == HequnPiecePlus::Piece(HequnPiece::White) && pieces[cx + x2][cy + y2] == HequnPiecePlus::Piece(HequnPiece::White) {
                        wr = true;
                    }
                }
                if br {
                    black_randian += 1;
                }
                if wr {
                    white_randian += 1;
                }
            }
        }
    }
    (black_randian - white_randian).abs() <= 1
}

fn evaluate_local(pieces: &Vec<Vec<HequnPiecePlus>>, cells: &Vec<Vec<HequnCell>>, cx: usize, cy: usize) -> f32 {
    let mut black_randian = 0;
    let mut white_randian = 0;
    let mut empty_num = 0;
    let mut black_num = 0;
    let mut white_num = 0;
    for dx in 0..=4 {
        for dy in 0..=4 {
            let (x, y) = (cx + dx, cy + dy);
            match pieces[x][y] {
                HequnPiecePlus::Piece(hequn_piece) => match hequn_piece {
                    HequnPiece::Black => { black_num += 1; },
                    HequnPiece::White => { white_num += 1; },
                },
                HequnPiecePlus::None => { 
                    empty_num += 1;
                    // 判断该格是否是双方染点
                    let mut br = false;
                    let mut wr = false;
                    for ((x1, y1), (x2, y2)) in get_local_randian()[dx][dy].clone() {
                        if pieces[cx + x1][cy + y1] == HequnPiecePlus::Piece(HequnPiece::Black) && pieces[cx + x2][cy + y2] == HequnPiecePlus::Piece(HequnPiece::Black) {
                            br = true;
                        }
                        if pieces[cx + x1][cy + y1] == HequnPiecePlus::Piece(HequnPiece::White) && pieces[cx + x2][cy + y2] == HequnPiecePlus::Piece(HequnPiece::White) {
                            wr = true;
                        }
                    }
                    if br {
                        black_randian += 1;
                    }
                    if wr {
                        white_randian += 1;
                    }
                },
                HequnPiecePlus::Wall => {},
            }
        }
    }
    // 如果染点数量不相等，根据染点数量判定
    if black_randian > white_randian {
        1.0
    } else if white_randian > black_randian {
        -1.0
    } else {
        match cells[cx][cy] {
            // 染点数量相等且未染色，如果该格附近空位已经不多，直接判定中立，否则根据该格附近棋子数判定
            HequnCell::Grey => {
                if empty_num >= 5 {
                    if black_num > white_num {
                        1.0
                    } else if black_num < white_num {
                        -1.0
                    } else {
                        0.0
                    }
                } else {
                    0.0
                }
            },
            // 染点数量相等且该格已被染色，根据已染色情况判定
            HequnCell::Colored(hequn_piece) => match hequn_piece {
                HequnPiece::Black => 1.0,
                HequnPiece::White => -1.0,
            },
        }
    }
}

pub fn evaluate(board: &HequnBoard) -> f32 {
    let mut res = 0.0;
    let mut pieces = vec![vec![HequnPiecePlus::Wall; 14]; 14];
    for x in 0..BOARD_SIZE_I {
        for y in 0..BOARD_SIZE_J {
            let xp = x + 2;
            let yp = y + 2;
            pieces[xp][yp] = match board.pieces[x][y] {
                Some(p) => HequnPiecePlus::Piece(p),
                None => HequnPiecePlus::None,
            };
        }
    }
    for x in 0..BOARD_SIZE_I {
        for y in 0..BOARD_SIZE_J {
            res += evaluate_local(&pieces, &board.cells, x, y);
        }
    }
    // 把目差估计转换为（先手方）胜率估计
    1.0 / (1.0 + (-res / TEMERATURE).exp())
}

const TEMERATURE: f32 = 10.0;

pub fn quick_move(board: &HequnBoard) -> Vec<HequnStep> {
    let mut step_results: Vec<(HequnStep, i32)> = Vec::new();
    
    let mut pieces = vec![vec![HequnPiecePlus::Wall; 14]; 14];
    for x in 0..BOARD_SIZE_I {
        for y in 0..BOARD_SIZE_J {
            let xp = x + 2;
            let yp = y + 2;
            pieces[xp][yp] = match board.pieces[x][y] {
                Some(p) => HequnPiecePlus::Piece(p),
                None => HequnPiecePlus::None,
            };
        }
    }

    let mut critical_num = vec![vec![0; 15]; 15];
    for x in 0..BOARD_SIZE_I {
        for y in 0..BOARD_SIZE_J {
            let xp = x + 3;
            let yp = y + 3;
            if evaluate_critical(&pieces, x, y) {
                critical_num[xp][yp] += 1;
            }
        }
    }
    for x in 0..15 {
        for y in 1..15 {
            critical_num[x][y] += critical_num[x][y-1];
        }
    }
    for x in 1..15 {
        for y in 0..15 {
            critical_num[x][y] += critical_num[x-1][y];
        }
    }

    step_results.push((HequnStep::Pass, 0));

    for x in 0..BOARD_SIZE_I {
        for y in 0..BOARD_SIZE_J {
            if board.pieces[x][y].is_some() {
                continue;
            }
            step_results.push((
                HequnStep::Pos(x, y), 
                critical_num[x+5][y+5] - critical_num[x][y+5] - critical_num[x+5][y] + critical_num[x][y]
            ));
        }
    }
        
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
}