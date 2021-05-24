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
}
