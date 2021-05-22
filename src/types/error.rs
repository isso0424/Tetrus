#![allow(dead_code)]
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum TetriminoError {
    #[error("Center is must be inside of tetrimino")]
    OutsideCenter {},
}
