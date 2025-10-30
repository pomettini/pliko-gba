use agb::input::Button;

use crate::scenario::ScenarioType;

#[derive(Clone, Copy)]
pub enum ActionType {
    Attack,
    Shield,
    Jump,
}

#[derive(Clone, Copy)]
pub struct Binding {
    pub button: Button,
    pub scenario: ScenarioType,
    pub action: ActionType,
}

pub const BINDINGS: &[Binding] = &[
    Binding {
        button: Button::L,
        scenario: ScenarioType::Water,
        action: ActionType::Attack,
    },
    Binding {
        button: Button::R,
        scenario: ScenarioType::Volcano,
        action: ActionType::Jump,
    },
    Binding {
        button: Button::B,
        scenario: ScenarioType::Swamp,
        action: ActionType::Shield,
    },
    Binding {
        button: Button::A,
        scenario: ScenarioType::Swamp,
        action: ActionType::Shield,
    },
];
