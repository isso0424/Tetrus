#![allow(dead_code)]
use crate::types::error::CursorError;
pub struct CursorHolder {
    cursor: (u8, u8),
    previous_command: CursorCommand,
}

pub enum CursorCommand {
    MoveLeft,
    MoveRight,
    MoveDown,
    Nothing,
}

impl CursorHolder {
    pub fn new(point: (u8, u8)) -> Self {
        Self {
            cursor: point,
            previous_command: CursorCommand::Nothing,
        }
    }

    pub fn exec(&mut self, cmd: CursorCommand) -> Result<(), CursorError> {
        match cmd {
            CursorCommand::MoveLeft => {
                if self.cursor.0 == 0 {
                    Err(CursorError::OutSideUnsigned {})
                } else {
                    self.cursor.0 -= 1;
                    Ok(())
                }
            }
            CursorCommand::MoveRight => {
                self.cursor.0 += 1;
                Ok(())
            }
            CursorCommand::MoveDown => {
                self.cursor.1 += 1;
                Ok(())
            }
            CursorCommand::Nothing => Ok(()),
        }
    }
}
