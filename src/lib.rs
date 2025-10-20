#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::display::Priority;
use agb::display::object::Object;
use agb::display::tiled::{RegularBackground, RegularBackgroundSize, TileFormat, VRAM_MANAGER};
use agb::input::{Button, ButtonController};
use agb::{include_aseprite, include_background_gfx};
use player::*;

use crate::gamestate::GameState;

pub mod gamestate;
pub mod player;
pub mod scenario;

include_background_gfx!(
    mod background,
    // game => deduplicate "gfx/background.png",
    blue => deduplicate "gfx/background-full-blue.png",
    red => deduplicate "gfx/background-full-red.png",
    green => deduplicate "gfx/background-full-green.png",
);

include_aseprite!(
    mod buttons,
    "gfx/blue_button.aseprite",
    "gfx/red_button.aseprite",
    "gfx/green_button.aseprite"
);

include_aseprite! {
    mod enemy,
    "gfx/enemy.aseprite"
}

pub enum ActionType {
    Attack,
    Shield,
    Jump,
}

pub fn main(mut gba: agb::Gba) -> ! {
    VRAM_MANAGER.set_background_palettes(background::PALETTES);

    let mut gfx = gba.graphics.get();

    let mut game_bg = RegularBackground::new(
        Priority::P3,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    let mut full_bg = RegularBackground::new(
        Priority::P1,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    let mut player = Player::new();

    let mut enemy = Object::new(enemy::IDLE.sprite(0));
    enemy.set_pos((130, 40));
    enemy.set_priority(Priority::P0);

    let mut button_left = Object::new(buttons::BLUE.sprite(0));
    button_left.set_pos((90 - 8, 135 - 7));

    let mut button_middle = Object::new(buttons::GREEN.sprite(0));
    button_middle.set_pos((111 - 8, 135 - 7));

    let mut button_right = Object::new(buttons::RED.sprite(0));
    button_right.set_pos((132 - 8, 135 - 7));

    game_bg.fill_with(&background::red);
    full_bg.fill_with(&background::red);

    let mut game = GameState::new();

    let mut counter = 0;

    loop {
        let mut input = ButtonController::new();

        let mut frame = gfx.frame();
        game_bg.show(&mut frame);
        full_bg.show(&mut frame);

        game.draw(&mut frame);

        if counter > 6 {
            player.update();
            counter = 0;
        }

        player.draw(&mut frame);

        enemy.show(&mut frame);

        button_left.show(&mut frame);
        button_middle.show(&mut frame);
        button_right.show(&mut frame);

        frame.commit();
        input.update();

        if input.is_just_pressed(Button::LEFT)
            || input.is_just_pressed(Button::UP)
            || input.is_just_pressed(Button::RIGHT)
            || input.is_just_pressed(Button::DOWN)
        {
            full_bg.fill_with(&background::blue);
            game.do_action(ActionType::Attack, &mut player);
        }

        if input.is_just_pressed(Button::B) {
            full_bg.fill_with(&background::red);
            game.do_action(ActionType::Shield, &mut player);
        }

        if input.is_just_pressed(Button::A) {
            full_bg.fill_with(&background::green);
            game.do_action(ActionType::Jump, &mut player);
        }

        counter += 1;
    }
}
