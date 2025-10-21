use agb::display::GraphicsFrame;

use crate::ActionType;
use crate::player::Player;
use crate::scenario::Scenario;

pub enum GameState {
    Game,
    GameOver,
}

pub struct Game {
    pub state: GameState,
    pub scenario: Scenario,
}

impl Game {
    pub fn new() -> Self {
        let mut scenario = Scenario::new();
        scenario.randomize();

        Self {
            state: GameState::Game,
            scenario,
        }
    }

    pub fn do_action(&mut self, action: ActionType, player: &mut Player) {
        player.perform_action(action);
        self.scenario.next();
    }

    pub fn draw(&mut self, frame: &mut GraphicsFrame<'_>) {
        self.scenario.draw(frame);
    }
}
