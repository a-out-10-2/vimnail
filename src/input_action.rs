use crate::action_type::ActionType;

use ggez::event::KeyCode;
pub type InputActions = std::vec::Vec<InputAction>;

#[derive(Clone)]
pub struct InputAction
{
    pub key_code: KeyCode,
    pub action_type: ActionType, 
}

impl InputAction {
    pub fn new(key_code: KeyCode, action_type: ActionType) -> Self {
        InputAction {
            key_code,
            action_type,
        }
    }
}