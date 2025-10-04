pub const BOARD_SIZE_I: usize = 10;
pub const BOARD_SIZE_J: usize = 10;
pub const PAINT_OFFSET: [((isize, isize), (isize, isize), (isize, isize)); 12] = [
    ((-2, 0), (-1, 0), (-1, 0)),
    ((2, 0), (1, 0), (1, 0)),
    ((0, -2), (0, -1), (0, -1)),
    ((0, 2), (0, 1), (0, 1)),
    ((-2, -2), (-1, -1), (-1, -1)),
    ((-2, 2), (-1, 1), (-1, 1)),
    ((2, -2), (1, -1), (1, -1)),
    ((2, 2), (1, 1), (1, 1)),
    ((-1, 0), (1, 0), (0, 0)),
    ((0, -1), (0, 1), (0, 0)),
    ((-1, -1), (1, 1), (0, 0)),
    ((-1, 1), (1, -1), (0, 0)),
];
pub const OFFSET: [(isize, isize); 9] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), (0, 0), (0, 1),
    (1, -1), (1, 0), (1, 1),
];

pub fn add_offset(from: (usize, usize), delta: (isize, isize)) -> Option<(usize, usize)> {
    let (from_x, from_y) = from;
    let (ifrom_x, ifrom_y) = (from_x as isize, from_y as isize);
    let (dx, dy) = delta;
    let (ito_x, ito_y) = (ifrom_x + dx, ifrom_y + dy);
    if ito_x < 0 || ito_x >= 10 || ito_y < 0 || ito_y >= 10 {
        return None
    }
    return Some((ito_x as usize, ito_y as usize))
}