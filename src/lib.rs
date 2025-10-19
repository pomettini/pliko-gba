#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::display::object::{Object, Tag};
use agb::display::tiled::{RegularBackground, RegularBackgroundSize, TileFormat, VRAM_MANAGER};
use agb::display::{GraphicsFrame, Priority};
use agb::input::{Button, ButtonController};
use agb::{include_aseprite, include_background_gfx, rng};
use player::*;

use crate::gamestate::GameState;

pub mod gamestate;
pub mod player;

include_background_gfx!(
    mod background,
    game => deduplicate "gfx/background.png",
);

include_aseprite! {
    mod enemy,
    "gfx/enemy.aseprite"
}

enum ActionType {
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

    let mut player = Player::new();

    let mut enemy = Object::new(enemy::IDLE.sprite(0));
    enemy.set_pos((130, 40));
    enemy.set_priority(Priority::P0);

    game_bg.fill_with(&background::game);

    let mut game = GameState::new();
    game.reset();

    loop {
        let mut input = ButtonController::new();

        let mut frame = gfx.frame();
        game_bg.show(&mut frame);

        game.draw(&mut frame);

        player.update();
        player.draw(&mut frame);

        enemy.show(&mut frame);

        frame.commit();
        input.update();

        if input.is_just_pressed(Button::LEFT)
            || input.is_just_pressed(Button::UP)
            || input.is_just_pressed(Button::RIGHT)
            || input.is_just_pressed(Button::DOWN)
        {
            game.do_action(ActionType::Attack, &mut player);
        }

        if input.is_just_pressed(Button::B) {
            game.do_action(ActionType::Shield, &mut player);
        }

        if input.is_just_pressed(Button::A) {
            game.do_action(ActionType::Jump, &mut player);
        }
    }
}
