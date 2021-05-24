#![allow(dead_code)]
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum TetriminoError {
    #[error("Center is must be inside of tetrimino")]
    OutsideCenter {},
    #[error("Cannot place to duplucate coordinate")]
    CannotPlaceDuplicate {},
}
