pub const BOARD_RADIUS: usize = 5;
pub const BOARD_DIAMETER: usize = 2 * BOARD_RADIUS - 1;
const IRADIUS: isize = BOARD_RADIUS as isize;
const IDIAMETER: isize = BOARD_DIAMETER as isize;

pub const OFFSET: [(isize, isize); 7] = [
    (-1, -1), (-1, 0), 
    (0, -1), (0, 0), (0, 1),
    (1, 0), (1, 1),
];

pub fn valid_coordinate(x: usize, y: usize) -> bool {
    let (x, y) = (x as isize, y as isize);
    return x >= 0 && y >= 0 && x < IDIAMETER && y < IDIAMETER && x - y < IRADIUS && y - x < IRADIUS;
}
pub fn add_offset(from: (usize, usize), delta: (isize, isize)) -> Option<(usize, usize)> {
    let (from_x, from_y) = from;
    let (ifrom_x, ifrom_y) = (from_x as isize, from_y as isize);
    let (dx, dy) = delta;
    let (ito_x, ito_y) = (ifrom_x + dx, ifrom_y + dy);
    if ito_x < 0 || ito_x >= IDIAMETER || ito_y < 0 || ito_y >= IDIAMETER || ito_x - ito_y >= IRADIUS || ito_y - ito_x >= IRADIUS {
        return None
    }
    return Some((ito_x as usize, ito_y as usize))
}