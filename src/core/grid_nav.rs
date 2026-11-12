// SPDX-License-Identifier: GPL-3.0-or-later

use super::appearance::Orientation;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug)]
pub struct Grid {
    pub columns: usize,
    pub rows: usize,
    pub count: usize,
    pub orientation: Orientation,
}

/// The new selected index. Wraps around at the edges; if the target cell is empty (incomplete last row or column),
/// moves to the last item.
pub fn step(index: usize, direction: Direction, grid: Grid) -> usize {
    if grid.count == 0 {
        return 0;
    }
    let stride = stride(direction, grid) as isize;
    let delta = match direction {
        Direction::Left | Direction::Up => -stride,
        Direction::Right | Direction::Down => stride,
    };
    wrap(index as isize + delta, grid.count)
}

/// Number of items to skip for the direction.
fn stride(direction: Direction, grid: Grid) -> usize {
    let across = matches!(direction, Direction::Up | Direction::Down);
    match (grid.orientation, across) {
        (Orientation::Horizontal, true) => grid.columns.max(1),
        (Orientation::Vertical, false) => grid.rows.max(1),
        _ => 1,
    }
}

fn wrap(index: isize, count: usize) -> usize {
    index.rem_euclid(count as isize) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const H: Grid = Grid { columns: 4, rows: 2, count: 8, orientation: Orientation::Horizontal };
    const V: Grid = Grid { columns: 2, rows: 4, count: 8, orientation: Orientation::Vertical };

    #[test]
    fn horizontal_up_down_moves_a_row() {
        assert_eq!(step(1, Direction::Down, H), 5);
        assert_eq!(step(5, Direction::Up, H), 1);
        assert_eq!(step(1, Direction::Right, H), 2);
    }

    #[test]
    fn vertical_left_right_moves_a_column() {
        assert_eq!(step(1, Direction::Right, V), 5);
        assert_eq!(step(5, Direction::Left, V), 1);
        assert_eq!(step(1, Direction::Down, V), 2);
    }

    #[test]
    fn wraps_around() {
        assert_eq!(step(0, Direction::Left, H), 7);
        assert_eq!(step(6, Direction::Down, H), 2);
    }

    #[test]
    fn single_row_behaves_like_a_list() {
        let g = Grid { columns: 5, rows: 1, count: 5, orientation: Orientation::Horizontal };
        assert_eq!(step(2, Direction::Down, g), 2);
    }
}
