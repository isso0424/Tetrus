use super::{error::TetriminoError, tetrimino::Tetrimino};
use std::convert::TryInto;

#[allow(dead_code)]
pub struct Board {
    pub minos: Vec<Vec<bool>>,
}

#[allow(dead_code)]
impl Board {
    pub fn new() -> Self {
        Board {
            minos: vec![vec![false; 10]; 20],
        }
    }

    pub fn place_mino(&mut self, mino: &Tetrimino, cursor: (u8, u8)) -> Result<(), TetriminoError> {
        let x = TryInto::<u8>::try_into(mino.shape.get(0).unwrap().len()).unwrap();
        let y = TryInto::<u8>::try_into(mino.shape.len()).unwrap();

        if cursor.0 < mino.center.0
            || TryInto::<u8>::try_into(cursor.0).unwrap() + x - mino.center.0 > 10
            || cursor.1 < mino.center.1
            || TryInto::<u8>::try_into(cursor.1).unwrap() + y - mino.center.1 > 20
        {
            return Err(TetriminoError::CannotPlaceDuplicate {});
        }

        mino.shape
            .iter()
            .enumerate()
            .map(|(y_pos, a)| {
                a.iter()
                    .enumerate()
                    .map(|(x_pos, b)| {
                        if *b
                            && self.minos[TryInto::<usize>::try_into(
                                cursor.1 + TryInto::<u8>::try_into(y_pos).unwrap() - mino.center.1,
                            )
                            .unwrap()][TryInto::<usize>::try_into(
                                cursor.0 + TryInto::<u8>::try_into(x_pos).unwrap() - mino.center.0,
                            )
                            .unwrap()]
                        {
                            Err(TetriminoError::CannotPlaceDuplicate {})
                        } else {
                            Ok(())
                        }
                    })
                    .rev()
                    .collect()
            })
            .rev()
            .flat_map::<Vec<Result<_, _>>, _>(|arr| arr)
            .rev()
            .collect::<Result<Vec<_>, _>>()?;

        mino.shape.iter().enumerate().for_each(|(y_pos, a)| {
            a.iter().enumerate().for_each(|(x_pos, b)| {
                if *b {
                    self.minos[TryInto::<usize>::try_into(
                        cursor.1 + TryInto::<u8>::try_into(y_pos).unwrap() - mino.center.1,
                    )
                    .unwrap()][TryInto::<usize>::try_into(
                        cursor.0 + TryInto::<u8>::try_into(x_pos).unwrap() - mino.center.0,
                    )
                    .unwrap()] = true;
                }
            })
        });
        Ok(())
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

        assert_eq!(board.place_mino(&test_mino, (1, 19)), Ok(()));
        assert_eq!(board.place_mino(&test_mino, (2, 19)), Ok(()));
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

        assert_ne!(board.place_mino(&test_mino, (10, 1)), Ok(()));
        assert_ne!(board.place_mino(&test_mino, (1, 20)), Ok(()));
        assert_ne!(board.place_mino(&test_mino, (0, 1)), Ok(()));
        assert_ne!(board.place_mino(&test_mino, (1, 0)), Ok(()));
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

        assert_eq!(board.place_mino(&test_mino, (1, 1)), Ok(()));
        assert_ne!(board.place_mino(&test_mino, (2, 2)), Ok(()));
    }
}
