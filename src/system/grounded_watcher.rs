#![allow(dead_code)]
use crate::types::board::Board;
use crate::types::tetrimino::Tetrimino;

pub struct GroundedWatcher<'a> {
    board: &'a Board,
}

impl<'a> GroundedWatcher<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self { board }
    }

    pub fn check(&self, mino: &Tetrimino, cursor: &(u8, u8)) -> bool {
        let places = Self::groundable_places(mino);
        let mut is_grounded = false;
        places.iter().enumerate().for_each(|(x_pos, y_pos)| {
            if *y_pos + 1 + cursor.1 as usize + mino.center.1 as usize >= self.board.minos.len()
                || self.board.minos[*y_pos + 1 + cursor.1 as usize + mino.center.1 as usize]
                    [x_pos + cursor.0 as usize + mino.center.0 as usize]
            {
                is_grounded = true;
            }
        });

        is_grounded
    }

    fn groundable_places(mino: &Tetrimino) -> Vec<usize> {
        let mut vx = vec![];
        mino.shape.iter().enumerate().for_each(|(y_pos, y)| {
            y.iter().enumerate().for_each(|(x_pos, x)| {
                if y_pos == 0 {
                    vx.push(0);
                } else if *x {
                    vx[x_pos] = y_pos;
                }
            })
        });

        vx
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn construct() {
        let board = Board::new();
        GroundedWatcher::new(&board);
    }

    #[test]
    fn check_grounded() {
        let mut board = Board::new();
        let b = board.clone();
        let watcher = GroundedWatcher::new(&b);
        let mino = Tetrimino::new(vec![vec![true, true], vec![true, true]], (0, 0)).unwrap();
        let result = watcher.check(&mino, &(0, 18));
        assert_eq!(result, true);

        let result = watcher.check(&mino, &(0, 16));
        assert_eq!(result, false);

        board.place_mino(&mino, &(0, 18)).unwrap();
        let new_board = board.clone();
        let watcher = GroundedWatcher::new(&new_board);
        let result = watcher.check(&mino, &(0, 16));
        assert_eq!(result, true);
        let result = watcher.check(&mino, &(0, 15));
        assert_eq!(result, false);
    }
}
