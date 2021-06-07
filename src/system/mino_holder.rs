#![allow(dead_code)]
use crate::types::tetrimino::Tetrimino;

pub struct MinoHolder<'a> {
    holded_mino: Option<&'a Tetrimino>,
}

impl<'a> MinoHolder<'a> {
    pub fn new() -> Self {
        Self { holded_mino: None }
    }

    pub fn hold(&mut self, mino: &'a Tetrimino) -> Option<&'a Tetrimino> {
        let tmp = self.holded_mino;
        self.holded_mino = Some(mino);
        tmp
    }
}
