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

pub struct Scenario {
    state: [ScenarioType; 4],
    small_sprite: [Option<Object>; 3],
    medium_sprite: [Option<Object>; 4],
    big_sprite: [Option<Object>; 4],
    full_sprite: [Option<Object>; 6],
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
            small_sprite: [const { None }; 3],
            medium_sprite: [const { None }; 4],
            big_sprite: [const { None }; 4],
            full_sprite: [const { None }; 6],
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

        let mut small_sprite = [
            Object::new(small.sprite(0)),
            Object::new(small.sprite(1)),
            Object::new(small.sprite(2)),
        ];

        small_sprite[0].set_pos((80, 8));
        small_sprite[1].set_pos((80 + 32, 8));
        small_sprite[2].set_pos((80 + 64, 8));

        self.small_sprite = small_sprite.map(Some);

        let medium = get_object(&ScenarioSize::Medium, &self.state[1]);

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

        let big = get_object(&ScenarioSize::Big, &self.state[2]);

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

        let full = get_object(&ScenarioSize::Full, &self.state[3]);

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

    pub fn draw(&mut self, frame: &mut GraphicsFrame<'_>) {
        for s in self.small_sprite.iter().flatten() {
            s.show(frame);
        }

        for s in self.medium_sprite.iter().flatten() {
            s.show(frame);
        }

        for s in self.big_sprite.iter().flatten() {
            s.show(frame);
        }

        for s in self.full_sprite.iter().flatten() {
            s.show(frame);
        }
    }
}
