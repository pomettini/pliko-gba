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

impl Player {
    pub fn new() -> Self {
        let mut player = Object::new(player::IDLE.sprite(0));
        player.set_pos((55, 86));
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
        match self.state {
            PlayerState::Idle => self.object.set_sprite(player::IDLE.sprite(self.anim_frame)),
            PlayerState::Attack => self.object.set_sprite(player::ATTACK.sprite(0)),
            PlayerState::Shield => self.object.set_sprite(player::SHIELD.sprite(0)),
            PlayerState::Jump => self.object.set_sprite(player::JUMP.sprite(0)),
        };
        self.object.set_pos((55, 86));
        self.object.set_priority(Priority::P0);

        self.anim_frame += 1;
        self.anim_frame %= 2;

        self.state = PlayerState::Idle;
    }

    pub fn draw(&self, frame: &mut GraphicsFrame<'_>) {
        self.object.show(frame);
    }
}
