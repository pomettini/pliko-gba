use agb::{
    display::{
        GraphicsFrame, Priority,
        object::{Object, Tag},
    },
    include_aseprite_256,
};

include_aseprite_256! {
    mod enemy_sprite,
    "gfx/enemy.aseprite"
}

pub struct Enemy<'a> {
    pub object: Object,
    pub tag: &'a Tag,
    pub anim_frame: usize,
}

impl<'a> Enemy<'a> {
    pub fn new(object: Object, tag: &'a Tag) -> Self {
        Self {
            object,
            tag,
            anim_frame: 0,
        }
    }

    pub fn update(&mut self) {
        self.anim_frame += 1;
        self.anim_frame %= 2;

        self.object.set_sprite(self.tag.sprite(self.anim_frame));
    }

    pub fn draw(&self, frame: &mut GraphicsFrame<'_>) {
        self.object.show(frame);
    }
}

pub fn setup_enemies<'a>() -> [Enemy<'a>; 4] {
    let mut enemy0 = Object::new(enemy_sprite::BIGROCKIDLE.sprite(0));
    enemy0.set_pos((121 + 60, 54));
    enemy0.set_priority(Priority::P3);

    let mut enemy1 = Object::new(enemy_sprite::LERCIOIDLE.sprite(0));
    enemy1.set_pos((121 + 40, 54));
    enemy1.set_priority(Priority::P2);

    let mut enemy2 = Object::new(enemy_sprite::GOBLINIDLE.sprite(0));
    enemy2.set_pos((121 + 20, 54));
    enemy2.set_priority(Priority::P1);

    let mut enemy3 = Object::new(enemy_sprite::MAGEIDLE.sprite(0));
    enemy3.set_pos((121, 54));
    enemy3.set_priority(Priority::P0);

    [
        Enemy::new(enemy0, &enemy_sprite::BIGROCKIDLE),
        Enemy::new(enemy1, &enemy_sprite::LERCIOIDLE),
        Enemy::new(enemy2, &enemy_sprite::GOBLINIDLE),
        Enemy::new(enemy3, &enemy_sprite::MAGEIDLE),
    ]
}
