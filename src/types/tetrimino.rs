use crate::types::error::TetriminoError;

pub struct Tetrimino {
    pub shape: Vec<Vec<bool>>,
    pub center: (u8, u8),
}

impl Tetrimino {
    pub fn new(shape: Vec<Vec<bool>>, center: (u8, u8)) -> Result<Self, TetriminoError> {
        if shape.iter().len() < center.0.into() || shape.iter().len() < center.1.into() {
            Err(TetriminoError::OutsideCenter {})
        } else {
            Ok(Self { shape, center })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_create_tetrimino() {
        let mino = Tetrimino::new(
            vec![vec![true, false, true], vec![false, true, false]],
            (1, 2),
        )
        .unwrap();
        assert_eq!(mino.shape.len(), 2);

        let first = mino.shape.iter().nth(0).unwrap();
        assert_eq!(first.len(), 3);
        assert_eq!(*first.iter().nth(0).unwrap(), true);
        assert_eq!(*first.iter().nth(1).unwrap(), false);
        assert_eq!(*first.iter().nth(2).unwrap(), true);
    }
}
