#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::display::Priority;
use agb::display::object::Object;
use agb::display::tiled::{RegularBackground, RegularBackgroundSize, TileFormat, VRAM_MANAGER};
use agb::{include_aseprite, include_background_gfx};

include_background_gfx!(
    mod background,
    game => deduplicate "gfx/background.png",
);

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

include_aseprite! {
    mod player,
    "gfx/player.aseprite"
}

include_aseprite! {
    mod enemy,
    "gfx/enemy.aseprite"
}

pub fn main(mut gba: agb::Gba) -> ! {
    VRAM_MANAGER.set_background_palettes(background::PALETTES);

    let mut gfx = gba.graphics.get();

    let mut game_bg = RegularBackground::new(
        Priority::P3,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    let mut small_sprite_0 = Object::new(green::SMALL.sprite(0));
    let mut small_sprite_1 = Object::new(green::SMALL.sprite(1));
    let mut small_sprite_2 = Object::new(green::SMALL.sprite(2));
    small_sprite_0.set_pos((80, 8));
    small_sprite_1.set_pos((80 + 32, 8));
    small_sprite_2.set_pos((80 + 64, 8));

    let mut medium_sprite_0 = Object::new(red::MEDIUM.sprite(0));
    let mut medium_sprite_1 = Object::new(red::MEDIUM.sprite(1));
    let mut medium_sprite_2 = Object::new(red::MEDIUM.sprite(2));
    let mut medium_sprite_3 = Object::new(red::MEDIUM.sprite(3));
    medium_sprite_0.set_pos((69, 16));
    medium_sprite_1.set_pos((69 + 32, 16));
    medium_sprite_2.set_pos((69 + 64, 16));
    medium_sprite_3.set_pos((69 + 96, 16));

    let mut big_sprite_0 = Object::new(blue::BIG.sprite(0));
    let mut big_sprite_1 = Object::new(blue::BIG.sprite(1));
    let mut big_sprite_2 = Object::new(blue::BIG.sprite(2));
    let mut big_sprite_3 = Object::new(blue::BIG.sprite(3));
    big_sprite_0.set_pos((56, 24));
    big_sprite_1.set_pos((56 + 32, 24));
    big_sprite_2.set_pos((56 + 64, 24));
    big_sprite_3.set_pos((56 + 96, 24));

    let mut full_sprite_0 = Object::new(blue::FULL.sprite(0));
    let mut full_sprite_1 = Object::new(blue::FULL.sprite(1));
    let mut full_sprite_2 = Object::new(blue::FULL.sprite(2));
    let mut full_sprite_3 = Object::new(blue::FULL.sprite(3));
    let mut full_sprite_4 = Object::new(blue::FULL.sprite(4));
    let mut full_sprite_5 = Object::new(blue::FULL.sprite(5));
    full_sprite_0.set_pos((40, 32));
    full_sprite_1.set_pos((40 + 64, 32));
    full_sprite_2.set_pos((40 + 128, 32));
    full_sprite_3.set_pos((40, 32 + 64));
    full_sprite_4.set_pos((40 + 64, 32 + 64));
    full_sprite_5.set_pos((40 + 128, 32 + 64));
    full_sprite_0.set_priority(Priority::P1);
    full_sprite_1.set_priority(Priority::P1);
    full_sprite_2.set_priority(Priority::P1);
    full_sprite_3.set_priority(Priority::P1);
    full_sprite_4.set_priority(Priority::P1);
    full_sprite_5.set_priority(Priority::P1);

    let mut player = Object::new(player::IDLE.sprite(0));
    player.set_pos((50, 60));
    player.set_priority(Priority::P0);

    let mut enemy = Object::new(enemy::IDLE.sprite(0));
    enemy.set_pos((130, 40));
    enemy.set_priority(Priority::P0);

    game_bg.fill_with(&background::game);

    loop {
        let mut frame = gfx.frame();
        game_bg.show(&mut frame);

        small_sprite_0.show(&mut frame);
        small_sprite_1.show(&mut frame);
        small_sprite_2.show(&mut frame);

        medium_sprite_0.show(&mut frame);
        medium_sprite_1.show(&mut frame);
        medium_sprite_2.show(&mut frame);
        medium_sprite_3.show(&mut frame);

        big_sprite_0.show(&mut frame);
        big_sprite_1.show(&mut frame);
        big_sprite_2.show(&mut frame);
        big_sprite_3.show(&mut frame);

        full_sprite_0.show(&mut frame);
        full_sprite_1.show(&mut frame);
        full_sprite_2.show(&mut frame);
        full_sprite_3.show(&mut frame);
        full_sprite_4.show(&mut frame);
        full_sprite_5.show(&mut frame);

        player.show(&mut frame);
        enemy.show(&mut frame);

        frame.commit();
    }
}
