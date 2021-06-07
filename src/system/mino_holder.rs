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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct() {
        let holder = MinoHolder::new();
        assert_eq!(holder.holded_mino.is_none(), true);
    }

    #[test]
    fn test_hold() {
        let mut holder = MinoHolder::new();
        let mino = Tetrimino::new(vec![vec![true, true], vec![true, true]], (0, 0)).unwrap();
        let fetched = holder.hold(&mino);
        assert_eq!(fetched.is_none(), true);
        let fetched = holder.hold(&mino).unwrap();
        assert_eq!(fetched.shape[0][0], true);
        assert_eq!(fetched.shape[0][1], true);
        assert_eq!(fetched.shape[1][0], true);
        assert_eq!(fetched.shape[1][1], true);
        assert_eq!(fetched.center.0, 0);
        assert_eq!(fetched.center.1, 0);
    }
}
