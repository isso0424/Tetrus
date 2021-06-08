#![allow(dead_code)]

use crate::types::board::Board;
pub struct Judge {
    banned_position: (u8, u8),
}

impl Judge {
    pub fn new(banned_position: (u8, u8)) -> Self {
        Self { banned_position }
    }

    pub fn check_end(&self, board: &Board) -> bool {
        board.minos[self.banned_position.1 as usize][self.banned_position.0 as usize]
    }
}

#[cfg(test)]
mod test {
    use crate::types::tetrimino::Tetrimino;

    use super::*;

    #[test]
    fn test_construct() {
        let judge = Judge::new((4, 0));
        assert_eq!(judge.banned_position.0, 4);
        assert_eq!(judge.banned_position.1, 0);
    }

    #[test]
    fn test_check_end() {
        let mut board = Board::new();
        let judge = Judge::new((4, 0));
        assert_eq!(judge.check_end(&board), false);
        board
            .place_mino(&Tetrimino::new(vec![vec![true]], (0, 0)).unwrap(), &(4, 0))
            .unwrap();
        assert_eq!(judge.check_end(&board), true);
    }
}
