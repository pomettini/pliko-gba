use agb::display::object::{Object, Tag};
use agb::display::tiled::{RegularBackground, RegularBackgroundSize, TileFormat, VRAM_MANAGER};
use agb::display::{GraphicsFrame, Priority};
use agb::input::{Button, ButtonController};
use agb::{include_aseprite, include_background_gfx, rng};

use crate::ActionType;
use crate::player::Player;

include_aseprite! {
    mod blue,
    "gfx/backgrounds-blue.aseprite",
    "gfx/background-blue-full.aseprite"
}

include_aseprite! {
    mod red,
    "gfx/backgrounds-red.aseprite",
    "gfx/background-red-full.aseprite"
}

include_aseprite! {
    mod green,
    "gfx/backgrounds-green.aseprite",
    "gfx/background-green-full.aseprite"
}

enum ScenarioSize {
    Small,
    Medium,
    Big,
    Full,
}

enum ScenarioType {
    Water,
    Volcano,
    Swamp,
}

fn get_random_small() -> &'static Tag {
    let elements = [&blue::SMALL, &green::SMALL, &red::SMALL];
    elements[(rng::next_i32() as usize) % 3]
}

fn get_random_medium() -> &'static Tag {
    let elements = [&blue::MEDIUM, &green::MEDIUM, &red::MEDIUM];
    elements[(rng::next_i32() as usize) % 3]
}

fn get_random_big() -> &'static Tag {
    let elements = [&blue::BIG, &green::BIG, &red::BIG];
    elements[(rng::next_i32() as usize) % 3]
}

fn get_random_full() -> &'static Tag {
    let elements = [&blue::FULL, &green::FULL, &red::FULL];
    elements[(rng::next_i32() as usize) % 3]
}

fn get_object(s_size: ScenarioSize, s_type: ScenarioType) -> &'static Tag {
    match s_type {
        ScenarioType::Water => match s_size {
            ScenarioSize::Small => &blue::SMALL,
            ScenarioSize::Medium => &blue::MEDIUM,
            ScenarioSize::Big => &blue::BIG,
            ScenarioSize::Full => &blue::FULL,
        },
        ScenarioType::Volcano => match s_size {
            ScenarioSize::Small => &red::SMALL,
            ScenarioSize::Medium => &red::MEDIUM,
            ScenarioSize::Big => &red::BIG,
            ScenarioSize::Full => &red::FULL,
        },
        ScenarioType::Swamp => match s_size {
            ScenarioSize::Small => &green::SMALL,
            ScenarioSize::Medium => &green::MEDIUM,
            ScenarioSize::Big => &green::BIG,
            ScenarioSize::Full => &green::FULL,
        },
    }
}

pub struct GameState {
    small_sprite: [Option<Object>; 3],
    medium_sprite: [Option<Object>; 4],
    big_sprite: [Option<Object>; 4],
    full_sprite: [Option<Object>; 6],
}

impl GameState {
    pub fn new() -> Self {
        Self {
            small_sprite: [const { None }; 3],
            medium_sprite: [const { None }; 4],
            big_sprite: [const { None }; 4],
            full_sprite: [const { None }; 6],
        }
    }

    pub fn reset(&mut self) {
        self.set_small(get_random_small());
        self.set_medium(get_random_medium());
        self.set_big(get_random_big());
        self.set_full(get_random_full());
    }

    pub fn set_small(&mut self, small: &Tag) {
        let mut small_sprite = [
            Object::new(small.sprite(0)),
            Object::new(small.sprite(1)),
            Object::new(small.sprite(2)),
        ];

        small_sprite[0].set_pos((80, 8));
        small_sprite[1].set_pos((80 + 32, 8));
        small_sprite[2].set_pos((80 + 64, 8));

        self.small_sprite = small_sprite.map(Some);
    }

    fn set_medium(&mut self, medium: &Tag) {
        let mut medium_sprite = [
            Object::new(medium.sprite(0)),
            Object::new(medium.sprite(1)),
            Object::new(medium.sprite(2)),
            Object::new(medium.sprite(3)),
        ];

        medium_sprite[0].set_pos((69, 16));
        medium_sprite[1].set_pos((69 + 32, 16));
        medium_sprite[2].set_pos((69 + 64, 16));
        medium_sprite[3].set_pos((69 + 96, 16));

        self.medium_sprite = medium_sprite.map(Some);
    }

    fn set_big(&mut self, big: &Tag) {
        let mut big_sprite = [
            Object::new(big.sprite(0)),
            Object::new(big.sprite(1)),
            Object::new(big.sprite(2)),
            Object::new(big.sprite(3)),
        ];

        big_sprite[0].set_pos((56, 24));
        big_sprite[1].set_pos((56 + 32, 24));
        big_sprite[2].set_pos((56 + 64, 24));
        big_sprite[3].set_pos((56 + 96, 24));

        self.big_sprite = big_sprite.map(Some);
    }

    fn set_full(&mut self, full: &Tag) {
        let mut full_sprite = [
            Object::new(full.sprite(0)),
            Object::new(full.sprite(1)),
            Object::new(full.sprite(2)),
            Object::new(full.sprite(3)),
            Object::new(full.sprite(4)),
            Object::new(full.sprite(5)),
        ];
        full_sprite[0].set_pos((40, 32));
        full_sprite[0].set_priority(Priority::P1);
        full_sprite[1].set_pos((40 + 64, 32));
        full_sprite[1].set_priority(Priority::P1);
        full_sprite[2].set_pos((40 + 128, 32));
        full_sprite[2].set_priority(Priority::P1);
        full_sprite[3].set_pos((40, 32 + 64));
        full_sprite[3].set_priority(Priority::P1);
        full_sprite[4].set_pos((40 + 64, 32 + 64));
        full_sprite[4].set_priority(Priority::P1);
        full_sprite[5].set_pos((40 + 128, 32 + 64));
        full_sprite[5].set_priority(Priority::P1);

        self.full_sprite = full_sprite.map(Some);
    }

    pub fn do_action(&mut self, action: ActionType, player: &mut Player) {
        player.perform_action(action);
        self.reset();
    }

    pub fn draw(&mut self, frame: &mut GraphicsFrame<'_>) {
        for sprite in &self.small_sprite {
            if let Some(s) = sprite {
                s.show(frame);
            }
        }

        for sprite in &self.medium_sprite {
            if let Some(s) = sprite {
                s.show(frame);
            }
        }

        for sprite in &self.big_sprite {
            if let Some(s) = sprite {
                s.show(frame);
            }
        }

        for sprite in &self.full_sprite {
            if let Some(s) = sprite {
                s.show(frame);
            }
        }
    }
}
