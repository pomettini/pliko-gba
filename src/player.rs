use agb::display::object::Object;
use agb::display::{GraphicsFrame, Priority};
use agb::include_aseprite;

use crate::ActionType;

include_aseprite! {
    mod player,
    "gfx/player.aseprite"
}

enum PlayerState {
    Idle,
    Attack,
    Shield,
    Jump,
}

pub struct Player {
    object: Object,
    state: PlayerState,
    anim_frame: usize,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Self {
        let mut player = Object::new(player::IDLE.sprite(0));
        player.set_pos((50, 60));
        player.set_priority(Priority::P0);

        Self {
            object: player,
            state: PlayerState::Idle,
            anim_frame: 0,
        }
    }

    pub fn perform_action(&mut self, action: ActionType) {
        self.state = match action {
            ActionType::Attack => PlayerState::Attack,
            ActionType::Shield => PlayerState::Shield,
            ActionType::Jump => PlayerState::Jump,
        }
    }

    pub fn update(&mut self) {
        self.object = match self.state {
            PlayerState::Idle => Object::new(player::IDLE.sprite(self.anim_frame)),
            PlayerState::Attack => Object::new(player::ATTACK.sprite(0)),
            PlayerState::Shield => Object::new(player::SHIELD.sprite(0)),
            PlayerState::Jump => Object::new(player::JUMP.sprite(0)),
        };
        self.object.set_pos((50, 60));
        self.object.set_priority(Priority::P0);

        self.anim_frame += 1;
        self.anim_frame %= 2;
    }

    pub fn draw(&self, frame: &mut GraphicsFrame<'_>) {
        self.object.show(frame);
    }
}
