use crate::types::error::TetriminoError;
use std::convert::TryInto;

#[allow(dead_code)]
pub struct Tetrimino {
    pub shape: Vec<Vec<bool>>,
    pub center: (u8, u8),
}

#[allow(dead_code)]
impl Tetrimino {
    pub fn new(shape: Vec<Vec<bool>>, center: (u8, u8)) -> Result<Self, TetriminoError> {
        if shape.iter().len() < center.1.into() || shape.get(0).unwrap().len() < center.0.into() {
            Err(TetriminoError::OutsideCenter {})
        } else {
            Ok(Self { shape, center })
        }
    }

    pub fn rotate(&self, is_right: bool) -> Self {
        let mut before: Vec<Vec<bool>> = self.shape.iter().cloned().collect();
        let x_len = before.get(0).unwrap().len();
        let y_len = before.iter().len();
        let mut shape = vec![];
        println!("{}", is_right);
        for n in 0..x_len {
            if !is_right {
                before.reverse();
            }
            let x_shape: Vec<bool> = before
                .iter()
                .map(|x| *x.get(if is_right { n } else { x_len - n - 1 }).unwrap())
                .rev()
                .collect();
            println!("{:?}", x_shape);
            shape.push(x_shape);
            if !is_right {
                before.reverse();
            }
        }

        let center = if is_right {
            (
                TryInto::<u8>::try_into(y_len).unwrap() - self.center.1 - 1,
                self.center.0,
            )
        } else {
            (
                self.center.1,
                TryInto::<u8>::try_into(x_len).unwrap() - self.center.0 - 1,
            )
        };

        Tetrimino { shape, center }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_create_tetrimino() {
        let mino = Tetrimino::new(
            vec![vec![true, false, true], vec![false, true, false]],
            (2, 1),
        )
        .unwrap();
        assert_eq!(mino.shape.len(), 2);

        let first = mino.shape.iter().nth(0).unwrap();
        assert_eq!(first.len(), 3);
        assert_eq!(*first.iter().nth(0).unwrap(), true);
        assert_eq!(*first.iter().nth(1).unwrap(), false);
        assert_eq!(*first.iter().nth(2).unwrap(), true);
    }

    #[test]
    fn check_right_rotate() {
        let mino = Tetrimino::new(
            vec![
                vec![true, true, false],
                vec![false, true, false],
                vec![false, true, false],
            ],
            (1, 2),
        )
        .unwrap();
        let mino = mino.rotate(true);
        let first = mino.shape.iter().nth(0).unwrap();
        assert_eq!(*first.iter().nth(0).unwrap(), false);
        assert_eq!(*first.iter().nth(1).unwrap(), false);
        assert_eq!(*first.iter().nth(2).unwrap(), true);

        assert_eq!(mino.center, (0, 1));

        let mino = mino.rotate(false);
        let second = mino.shape.iter().nth(0).unwrap();
        assert_eq!(*second.iter().nth(0).unwrap(), true);
        assert_eq!(*second.iter().nth(1).unwrap(), true);
        assert_eq!(*second.iter().nth(2).unwrap(), false);

        assert_eq!(mino.center, (1, 2));

        let mino = mino.rotate(false);
        let third = mino.shape.iter().nth(0).unwrap();
        assert_eq!(*third.iter().nth(0).unwrap(), false);
        assert_eq!(*third.iter().nth(1).unwrap(), false);
        assert_eq!(*third.iter().nth(2).unwrap(), false);

        assert_eq!(mino.center, (2, 1));
    }
}
