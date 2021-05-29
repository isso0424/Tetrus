use crate::types::tetrimino::Tetrimino;
use rand::Rng;
use std::cell::Cell;

pub struct NextMino {
    minos: Cell<Vec<Tetrimino>>,
}

impl NextMino {
    pub fn new(minos_patterns: Vec<Tetrimino>, max: usize) -> Self {
        let length = minos_patterns.len();
        let mut rng = rand::thread_rng();

        let mut mino_list = vec![];
        for _ in (0..max) {
            mino_list.push(minos_patterns[rng.gen_range(0..length)].clone());
        }
        Self {
            minos: Cell::new(mino_list),
        }
    }
}
