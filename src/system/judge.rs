#![allow(dead_code)]

use crate::types::board::Board;
pub struct Judge<'a> {
    board: &'a Board,
    banned_position: (u8, u8),
}

impl<'a> Judge<'a> {
    pub fn new(board: &'a Board, banned_position: (u8, u8)) -> Self {
        Self {
            board,
            banned_position,
        }
    }

    pub fn check_end(&self) -> bool {
        self.board.minos[self.banned_position.0 as usize][self.banned_position.1 as usize]
    }
}
