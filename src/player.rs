use agb::display::object::{AffineMode, ObjectAffine, SpriteVram};
use agb::display::{AffineMatrix, GraphicsFrame};
use agb::fixnum::{Num, Vector2D, num, vec2};
use agb::include_aseprite;

use crate::ActionType;

include_aseprite! {
    mod player,
    "gfx/player.aseprite"
}

#[derive(PartialEq)]
pub enum PlayerState {
    Idle,
    Attack,
    Shield,
    Jump,
    Dead,
}

pub struct Player {
    counter: Num<i32, 8>,
    sprite: SpriteVram,
    state: PlayerState,
    accumulator: usize,
    anim_frame: usize,
}

impl Player {
    pub fn new() -> Self {
        let sprite: SpriteVram = player::IDLE.sprite(0).into();
        Self {
            counter: num!(0.0),
            sprite,
            state: PlayerState::Idle,
            accumulator: 0,
            anim_frame: 0,
        }
    }

    pub fn reset(&mut self) {
        self.state = PlayerState::Idle;
        self.counter = num!(0.0);
    }

    pub fn perform_action(&mut self, action: ActionType) {
        self.accumulator = 0;
        self.state = match action {
            ActionType::Attack => PlayerState::Attack,
            ActionType::Shield => PlayerState::Shield,
            ActionType::Jump => PlayerState::Jump,
        }
    }

    pub fn kill(&mut self) {
        self.state = PlayerState::Dead;
    }

    pub fn is_dead(&self) -> bool {
        self.state == PlayerState::Dead
    }

    pub fn update(&mut self) {
        self.sprite = match self.state {
            PlayerState::Idle => player::IDLE.sprite(self.anim_frame).into(),
            PlayerState::Attack => player::ATTACK.sprite(0).into(),
            PlayerState::Shield => player::SHIELD.sprite(0).into(),
            PlayerState::Jump => player::JUMP.sprite(0).into(),
            PlayerState::Dead => player::DEATH.sprite(0).into(),
        };

        if self.state == PlayerState::Dead {
            self.counter -= num!(0.1);
            return;
        }

        if self.accumulator > 6 {
            self.anim_frame += 1;
            self.anim_frame %= 2;
            self.state = PlayerState::Idle;
            self.accumulator = 0;
        }

        self.accumulator += 1;
    }

    pub fn draw(&self, frame: &mut GraphicsFrame<'_>) {
        let test = num!(1.0) + ((self.counter / 10) % num!(0.5));

        let position: Vector2D<Num<i32, 8>> = match self.state {
            PlayerState::Dead => vec2((36 - 6).into(), (71 + 3).into()),
            _ => vec2(36.into(), 71.into()),
        };

        let rot_mat: AffineMatrix = AffineMatrix::from_rotation(self.counter);
        let scale_mat: AffineMatrix = AffineMatrix::from_scale(vec2(test, test));
        let pos_mat: AffineMatrix = AffineMatrix::from_translation(position);

        let final_transform: AffineMatrix = pos_mat * rot_mat * scale_mat;

        ObjectAffine::new(
            self.sprite.clone(),
            final_transform.into(),
            AffineMode::AffineDouble,
        )
        .set_pos(final_transform.position().round())
        .show(frame);
    }
}
