use agb::{
    display::{
        GraphicsFrame, Priority,
        object::{Object, Tag},
    },
    include_aseprite, rng,
};

include_aseprite! {
    mod blue,
    "gfx/backgrounds-blue.aseprite",
    // "gfx/background-blue-full.aseprite"
}

include_aseprite! {
    mod red,
    "gfx/backgrounds-red.aseprite",
    // "gfx/background-red-full.aseprite"
}

include_aseprite! {
    mod green,
    "gfx/backgrounds-green.aseprite",
    // "gfx/background-green-full.aseprite"
}

enum ScenarioSize {
    Small,
    Medium,
    Big,
    // Full,
}

#[derive(Copy, Clone)]
enum ScenarioType {
    Water,
    Volcano,
    Swamp,
}

const fn get_object(s_size: &ScenarioSize, s_type: &ScenarioType) -> &'static Tag {
    match s_type {
        ScenarioType::Water => match s_size {
            ScenarioSize::Small => &blue::SMALL,
            ScenarioSize::Medium => &blue::MEDIUM,
            ScenarioSize::Big => &blue::BIG,
            // ScenarioSize::Full => &blue::FULL,
        },
        ScenarioType::Volcano => match s_size {
            ScenarioSize::Small => &red::SMALL,
            ScenarioSize::Medium => &red::MEDIUM,
            ScenarioSize::Big => &red::BIG,
            // ScenarioSize::Full => &red::FULL,
        },
        ScenarioType::Swamp => match s_size {
            ScenarioSize::Small => &green::SMALL,
            ScenarioSize::Medium => &green::MEDIUM,
            ScenarioSize::Big => &green::BIG,
            // ScenarioSize::Full => &green::FULL,
        },
    }
}

pub struct Scenario {
    state: [ScenarioType; 4],
    small_sprite: [Object; 3],
    medium_sprite: [Object; 4],
    big_sprite: [Object; 4],
    // full_sprite: [Object; 6],
}

impl Scenario {
    pub fn new() -> Self {
        Self {
            state: [
                ScenarioType::Water,
                ScenarioType::Water,
                ScenarioType::Water,
                ScenarioType::Water,
            ],
            small_sprite: [
                Object::new(blue::SMALL.sprite(0)),
                Object::new(blue::SMALL.sprite(1)),
                Object::new(blue::SMALL.sprite(2)),
            ],
            medium_sprite: [
                Object::new(blue::MEDIUM.sprite(0)),
                Object::new(blue::MEDIUM.sprite(1)),
                Object::new(blue::MEDIUM.sprite(2)),
                Object::new(blue::MEDIUM.sprite(3)),
            ],
            big_sprite: [
                Object::new(blue::BIG.sprite(0)),
                Object::new(blue::BIG.sprite(1)),
                Object::new(blue::BIG.sprite(2)),
                Object::new(blue::BIG.sprite(3)),
            ],
            /*
            full_sprite: [
                Object::new(blue::FULL.sprite(0)),
                Object::new(blue::FULL.sprite(1)),
                Object::new(blue::FULL.sprite(2)),
                Object::new(blue::FULL.sprite(3)),
                Object::new(blue::FULL.sprite(4)),
                Object::new(blue::FULL.sprite(5)),
            ],
            */
        }
    }

    pub fn randomize(&mut self) {
        let scenarios = [
            ScenarioType::Water,
            ScenarioType::Volcano,
            ScenarioType::Swamp,
        ];

        self.state = [
            scenarios[rng::next_i32() as usize % 3],
            scenarios[rng::next_i32() as usize % 3],
            scenarios[rng::next_i32() as usize % 3],
            scenarios[rng::next_i32() as usize % 3],
        ];
    }

    pub fn next(&mut self) {
        let scenarios = [
            ScenarioType::Water,
            ScenarioType::Volcano,
            ScenarioType::Swamp,
        ];

        self.state.rotate_right(1);
        self.state[0] = scenarios[rng::next_i32() as usize % 3];
    }

    pub fn assign(&mut self) {
        let small = get_object(&ScenarioSize::Small, &self.state[0]);

        self.small_sprite[0].set_sprite(small.sprite(0));
        self.small_sprite[1].set_sprite(small.sprite(1));
        self.small_sprite[2].set_sprite(small.sprite(2));

        self.small_sprite[0].set_pos((80, 8));
        self.small_sprite[1].set_pos((80 + 32, 8));
        self.small_sprite[2].set_pos((80 + 64, 8));

        let medium = get_object(&ScenarioSize::Medium, &self.state[1]);

        self.medium_sprite[0].set_sprite(medium.sprite(0));
        self.medium_sprite[1].set_sprite(medium.sprite(1));
        self.medium_sprite[2].set_sprite(medium.sprite(2));
        self.medium_sprite[3].set_sprite(medium.sprite(3));

        self.medium_sprite[0].set_pos((69, 16));
        self.medium_sprite[1].set_pos((69 + 32, 16));
        self.medium_sprite[2].set_pos((69 + 64, 16));
        self.medium_sprite[3].set_pos((69 + 96, 16));

        let big = get_object(&ScenarioSize::Big, &self.state[2]);

        self.big_sprite[0].set_sprite(big.sprite(0));
        self.big_sprite[1].set_sprite(big.sprite(1));
        self.big_sprite[2].set_sprite(big.sprite(2));
        self.big_sprite[3].set_sprite(big.sprite(3));

        self.big_sprite[0].set_pos((56, 24));
        self.big_sprite[1].set_pos((56 + 32, 24));
        self.big_sprite[2].set_pos((56 + 64, 24));
        self.big_sprite[3].set_pos((56 + 96, 24));

        /*
        let full = get_object(&ScenarioSize::Full, &self.state[3]);

        self.full_sprite[0].set_sprite(full.sprite(0));
        self.full_sprite[1].set_sprite(full.sprite(1));
        self.full_sprite[2].set_sprite(full.sprite(2));
        self.full_sprite[3].set_sprite(full.sprite(3));
        self.full_sprite[4].set_sprite(full.sprite(4));
        self.full_sprite[5].set_sprite(full.sprite(5));

        self.full_sprite[0].set_pos((40, 32));
        self.full_sprite[0].set_priority(Priority::P1);
        self.full_sprite[1].set_pos((40 + 64, 32));
        self.full_sprite[1].set_priority(Priority::P1);
        self.full_sprite[2].set_pos((40 + 128, 32));
        self.full_sprite[2].set_priority(Priority::P1);
        self.full_sprite[3].set_pos((40, 32 + 64));
        self.full_sprite[3].set_priority(Priority::P1);
        self.full_sprite[4].set_pos((40 + 64, 32 + 64));
        self.full_sprite[4].set_priority(Priority::P1);
        self.full_sprite[5].set_pos((40 + 128, 32 + 64));
        self.full_sprite[5].set_priority(Priority::P1);
        */
    }

    pub fn draw(&mut self, frame: &mut GraphicsFrame<'_>) {
        for sprite in &self.small_sprite {
            sprite.show(frame);
        }

        for sprite in &self.medium_sprite {
            sprite.show(frame);
        }

        for sprite in &self.big_sprite {
            sprite.show(frame);
        }

        /*
        for sprite in &self.full_sprite {
            sprite.show(frame);
        }
        */
    }
}
