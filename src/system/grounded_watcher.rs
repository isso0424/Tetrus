#![allow(dead_code)]
use crate::types::board::Board;

pub struct GroundedWatcher<'a> {
    board: &'a Board,
}

impl<'a> GroundedWatcher<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self { board }
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
