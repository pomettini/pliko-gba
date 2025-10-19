use agb::display::object::{Object, Tag};
use agb::display::{GraphicsFrame, Priority};
use agb::{include_aseprite, rng};

use crate::ActionType;
use crate::player::Player;
use crate::scenario::Scenario;

pub struct GameState {
    scenario: Scenario,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            scenario: Scenario::new(),
        }
    }

    pub fn do_action(&mut self, action: ActionType, player: &mut Player) {
        player.perform_action(action);
        self.scenario.reset();
    }

    pub fn reset(&mut self) {
        self.scenario.reset();
    }

    pub fn draw(&mut self, frame: &mut GraphicsFrame<'_>) {
        self.scenario.draw(frame);
    }
}
