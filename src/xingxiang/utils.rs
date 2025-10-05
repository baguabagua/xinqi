pub const BOARD_SIZE_I: usize = 8;
pub const BOARD_SIZE_J: usize = 8;

pub const OFFSET_KING: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), (0, 1),
    (1, -1), (1, 0), (1, 1),
];
pub const OFFSET_KNIGHT: [(isize, isize); 8] = [
    (-2, -1), (-2, 1), (-1, -2), (-1, 2),
    (1, -2), (1, 2), (2, -1), (2, 1),
];
pub const OFFSET_ROOK: [(isize, isize); 8] = [
    (-2, 0), (-1, -1), (-1, 1), 
    (0, -2), (0, 2), 
    (1, -1), (1, 1), (2, 0),
];
pub const OFFSET_BISHOP: [(isize, isize); 8] = [
    (-2, -2), (-2, 2), (-1, 0),
    (0, -1), (0, 1),
    (1, 0), (2, -2), (2, 2),
];

pub fn valid_coordinate(x: usize, y: usize) -> bool {
    return x < BOARD_SIZE_I && y < BOARD_SIZE_J
}
pub fn add_offset(from: (usize, usize), delta: (isize, isize)) -> Option<(usize, usize)> {
    let (from_x, from_y) = from;
    let (ifrom_x, ifrom_y) = (from_x as isize, from_y as isize);
    let (dx, dy) = delta;
    let (ito_x, ito_y) = (ifrom_x + dx, ifrom_y + dy);
    if ito_x < 0 || ito_x >= BOARD_SIZE_I as isize || ito_y < 0 || ito_y >= BOARD_SIZE_J as isize {
        return None
    }
    return Some((ito_x as usize, ito_y as usize))
}
pub fn diff(from: (usize, usize), to: (usize, usize)) -> (isize, isize) {
    let (from_x, from_y) = from;
    let (ifrom_x, ifrom_y) = (from_x as isize, from_y as isize);
    let (to_x, to_y) = to;
    let (ito_x, ito_y) = (to_x as isize, to_y as isize);
    return (ito_x - ifrom_x, ito_y - ifrom_y);
}