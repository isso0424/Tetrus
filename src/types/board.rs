use super::{error::TetriminoError, tetrimino::Tetrimino};

pub struct Board {
    pub minos: Vec<Vec<bool>>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            minos: vec![vec![false; 10]; 20],
        }
    }

    pub fn place_mino(
        &mut self,
        mino: &Tetrimino,
        cursor: &(u8, u8),
    ) -> Result<(), TetriminoError> {
        if !self.check_placeable(mino, cursor) {
            return Err(TetriminoError::CannotPlaceDuplicate {});
        }
        mino.shape.iter().enumerate().for_each(|(y_pos, a)| {
            a.iter().enumerate().for_each(|(x_pos, b)| {
                if *b {
                    self.minos[(cursor.1 + y_pos as u8 - mino.center.1) as usize]
                        [(cursor.0 + x_pos as u8 - mino.center.0) as usize] = true;
                }
            })
        });
        let delete_targets = self.minos.iter().map(|a| a.iter().all(|x| *x)).collect();
        self.delete_line(delete_targets);
        Ok(())
    }

    pub fn check_placeable(&self, mino: &Tetrimino, cursor: &(u8, u8)) -> bool {
        let x = mino.shape.get(0).unwrap().len();
        let y = mino.shape.len();

        if cursor.0 < mino.center.0
            || cursor.0 as usize + x - mino.center.0 as usize > 10
            || cursor.1 < mino.center.1
            || cursor.1 as usize + y - mino.center.1 as usize > 20
        {
            return false;
        }

        mino.shape
            .iter()
            .enumerate()
            .map::<Vec<Result<(), TetriminoError>>, _>(|(y_pos, a)| {
                a.iter()
                    .enumerate()
                    .map(|(x_pos, b)| {
                        if *b
                            && self.minos[cursor.1 as usize + y_pos - mino.center.1 as usize]
                                [cursor.0 as usize + x_pos - mino.center.0 as usize]
                        {
                            Err(TetriminoError::CannotPlaceDuplicate {})
                        } else {
                            Ok(())
                        }
                    })
                    .collect()
            })
            .flatten()
            .collect::<Result<Vec<_>, _>>()
            .is_ok()
    }

    fn delete_line(&mut self, delete_targets: Vec<bool>) {
        delete_targets.iter().enumerate().for_each(|(y_pos, cond)| {
            if *cond {
                self.minos.remove(y_pos);
                self.minos.insert(0, vec![false; 10])
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_constructor() {
        let board = Board::new();

        assert_eq!(board.minos.len(), 20);
        assert_eq!(board.minos.get(0).unwrap().len(), 10);
        board
            .minos
            .iter()
            .for_each(|x| x.iter().for_each(|y| assert_eq!(*y, false)));
    }

    #[test]
    fn place_mino() {
        let mut board = Board::new();
        let test_mino = super::super::tetrimino::Tetrimino::new(
            vec![
                vec![true, false, true, false, true],
                vec![false, true, false, true, false],
            ],
            (1, 1),
        )
        .unwrap();

        assert_eq!(board.place_mino(&test_mino, &(1, 19)), Ok(()));
        assert_eq!(board.place_mino(&test_mino, &(2, 19)), Ok(()));
    }

    #[test]
    fn place_to_outer_of_board() {
        let mut board = Board::new();
        let test_mino = super::super::tetrimino::Tetrimino::new(
            vec![
                vec![true, true, true],
                vec![true, true, true],
                vec![true, true, true],
            ],
            (1, 1),
        )
        .unwrap();

        assert_ne!(board.place_mino(&test_mino, &(10, 1)), Ok(()));
        assert_ne!(board.place_mino(&test_mino, &(1, 20)), Ok(()));
        assert_ne!(board.place_mino(&test_mino, &(0, 1)), Ok(()));
        assert_ne!(board.place_mino(&test_mino, &(1, 0)), Ok(()));
    }

    #[test]
    fn place_to_duplicate_place() {
        let mut board = Board::new();
        let test_mino = super::super::tetrimino::Tetrimino::new(
            vec![
                vec![true, true, true],
                vec![true, true, true],
                vec![true, true, true],
            ],
            (1, 1),
        )
        .unwrap();

        assert_eq!(board.place_mino(&test_mino, &(1, 1)), Ok(()));
        assert_ne!(board.place_mino(&test_mino, &(2, 2)), Ok(()));
    }

    #[test]
    fn delete_line() {
        let mut board = Board::new();
        let test_mino =
            super::super::tetrimino::Tetrimino::new(vec![vec![true; 10]; 2], (0, 0)).unwrap();

        assert_eq!(board.place_mino(&test_mino, &(0, 0)), Ok(()));
        assert_eq!(board.minos.iter().all(|x| x.iter().all(|y| !*y)), true);

        let test_mino =
            super::super::tetrimino::Tetrimino::new(vec![vec![true; 5]], (0, 0)).unwrap();
        assert_eq!(board.place_mino(&test_mino, &(0, 0)), Ok(()));
        assert_eq!(board.place_mino(&test_mino, &(5, 0)), Ok(()));
        assert_eq!(board.minos.iter().all(|x| x.iter().all(|y| !*y)), true);
    }
}
