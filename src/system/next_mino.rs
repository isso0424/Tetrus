use crate::types::tetrimino::Tetrimino;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cell::Cell;

pub struct NextMino {
    minos: Cell<Vec<Tetrimino>>,
    patterns: Vec<Tetrimino>,
    rnd_seed: Cell<ThreadRng>,
}

impl NextMino {
    pub fn new(minos_patterns: Vec<Tetrimino>, max: usize) -> Self {
        let length = minos_patterns.len();
        let mut rng = rand::thread_rng();

        let mut mino_list = vec![];
        for _ in 0..max {
            mino_list.push(minos_patterns[rng.gen_range(0..length)].clone());
        }
        Self {
            minos: Cell::new(mino_list),
            patterns: minos_patterns,
            rnd_seed: Cell::new(rng),
        }
    }

    fn get(&mut self) -> Tetrimino {
        let length = self.patterns.len();
        self.minos
            .get_mut()
            .push(self.patterns[self.rnd_seed.get_mut().gen_range(0..length)].clone());

        self.minos.get_mut().remove(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_create_next_minos() {
        let minos = vec![
            Tetrimino::new(vec![vec![true; 2]; 2], (1, 1)).unwrap(),
            Tetrimino::new(
                vec![vec![true, false, false], vec![true, true, true]],
                (1, 1),
            )
            .unwrap(),
        ];
        let mut next_mino_sys = NextMino::new(minos, 4);
        assert_eq!(next_mino_sys.minos.get_mut().len(), 4);
    }
}
