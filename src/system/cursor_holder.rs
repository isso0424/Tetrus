#![allow(dead_code)]
use crate::types::error::CursorError;
#[derive(Debug)]
pub struct CursorHolder {
    cursor: (u8, u8),
    previous_command: CursorCommand,
}

#[derive(Debug)]
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

    pub fn exec(&mut self, cmd: CursorCommand) -> Result<&mut Self, CursorError> {
        match cmd {
            CursorCommand::MoveLeft => {
                if self.cursor.0 == 0 {
                    Err(CursorError::OutSideUnsigned {})
                } else {
                    self.cursor.0 -= 1;
                    Ok(self)
                }
            }
            CursorCommand::MoveRight => {
                self.cursor.0 += 1;
                Ok(self)
            }
            CursorCommand::MoveDown => {
                self.cursor.1 += 1;
                Ok(self)
            }
            CursorCommand::Nothing => Ok(self),
        }
    }

    pub fn get_cursor(&self) -> (u8, u8) {
        self.cursor
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn construct() {
        let cursor_holder = CursorHolder::new((0, 0));
        assert_eq!(cursor_holder.get_cursor(), (0, 0));
    }

    #[test]
    fn test_move() {
        let mut cursor_holder = CursorHolder::new((0, 0));
        assert_eq!(cursor_holder.exec(CursorCommand::MoveLeft).is_ok(), false);

        cursor_holder.exec(CursorCommand::MoveDown).unwrap();
        assert_eq!(cursor_holder.get_cursor().0, 0);
        assert_eq!(cursor_holder.get_cursor().1, 1);

        cursor_holder.exec(CursorCommand::MoveRight).unwrap();
        assert_eq!(cursor_holder.get_cursor().0, 1);
        assert_eq!(cursor_holder.get_cursor().1, 1);

        cursor_holder.exec(CursorCommand::MoveLeft).unwrap();
        assert_eq!(cursor_holder.get_cursor().0, 0);
        assert_eq!(cursor_holder.get_cursor().1, 1);

        cursor_holder.exec(CursorCommand::Nothing).unwrap();
        assert_eq!(cursor_holder.get_cursor().0, 0);
        assert_eq!(cursor_holder.get_cursor().1, 1);
    }
}
