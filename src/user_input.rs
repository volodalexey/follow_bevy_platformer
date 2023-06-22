use bevy::prelude::KeyCode;
use leafwing_input_manager::{prelude::InputMap, Actionlike};

#[derive(Debug, Actionlike, Clone)]
pub enum PlayerInput {
    Left,
    Right,
    Jump,
    Fall,
    NextPlayer,
    PevPlayer,
}

impl PlayerInput {
    pub fn player_one() -> InputMap<PlayerInput> {
        let mut map = InputMap::default();
        map.insert_multiple([
            (KeyCode::A, PlayerInput::Left),
            (KeyCode::Left, PlayerInput::Left),
            (KeyCode::D, PlayerInput::Right),
            (KeyCode::Right, PlayerInput::Right),
            (KeyCode::W, PlayerInput::Jump),
            (KeyCode::Space, PlayerInput::Jump),
            (KeyCode::Up, PlayerInput::Jump),
            (KeyCode::S, PlayerInput::Fall),
            (KeyCode::Down, PlayerInput::Fall),
            (KeyCode::Q, PlayerInput::PevPlayer),
            (KeyCode::E, PlayerInput::NextPlayer),
        ]);
        map
    }
}
