use agb::display::GraphicsFrame;

use crate::ActionType;
use crate::player::Player;
use crate::scenario::Scenario;

pub struct GameState {
    scenario: Scenario,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        let mut scenario = Scenario::new();
        scenario.randomize();
        scenario.setup();

        Self { scenario }
    }

    pub fn do_action(&mut self, action: ActionType, player: &mut Player) {
        player.perform_action(action);
        self.scenario.randomize();
        self.scenario.setup();
    }

    pub fn draw(&mut self, frame: &mut GraphicsFrame<'_>) {
        self.scenario.draw(frame);
    }
}
