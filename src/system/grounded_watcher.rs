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
            if *y_pos + 1 + cursor.1 as usize > mino.shape.len() {
                is_grounded = true;
            } else if mino.shape[*y_pos + 1 + cursor.1 as usize][x_pos] {
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
    fn construct() {
        let board = Board::new();
        GroundedWatcher::new(&board);
    }
}
