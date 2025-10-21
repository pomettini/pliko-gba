#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::display::font::{AlignmentKind, Font, Layout, ObjectTextRenderer};
use agb::display::object::{Object, Size};
use agb::display::tiled::{RegularBackground, RegularBackgroundSize, TileFormat, VRAM_MANAGER};
use agb::display::{Palette16, Priority, Rgb15};
use agb::fixnum::vec2;
use agb::input::{Button, ButtonController};
use agb::sound::mixer::{Frequency, SoundChannel, SoundData};
use agb::{include_aseprite, include_background_gfx, include_font, include_wav, println};
use player::*;

extern crate alloc;
use alloc::vec::Vec;

use crate::gamestate::{Game, GameState};
use crate::scenario::{Scenario, ScenarioType};

pub mod gamestate;
pub mod player;
pub mod scenario;

include_background_gfx!(
    mod background,
    game => deduplicate "gfx/background.png",
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

pub fn update_full_background(scenario: &Scenario, background: &mut RegularBackground) {
    let bg = match scenario.state[3] {
        ScenarioType::Water => &background::blue,
        ScenarioType::Volcano => &background::red,
        ScenarioType::Swamp => &background::green,
    };
    background.fill_with(&bg);
}

pub fn main(mut gba: agb::Gba) -> ! {
    static PALETTE: &Palette16 = const {
        let mut palette = [Rgb15::BLACK; 16];
        palette[1] = Rgb15::WHITE;
        palette[2] = Rgb15(0x10_7C);
        &Palette16::new(palette)
    };

    static FONT: Font = include_font!("gfx/pixelated.ttf", 10);

    let layout = Layout::new(
        "Game over!\nPress Start to restart",
        &FONT,
        AlignmentKind::Left,
        16,
        200,
    );

    let text_render = ObjectTextRenderer::new(PALETTE.into(), Size::S16x16);

    let objects: Vec<_> = layout.map(|x| text_render.show(&x, vec2(16, 16))).collect();

    // VRAM_MANAGER.set_background_palettes(background::PALETTES);

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

    game_bg.fill_with(&background::game);

    let mut game = Game::new();
    update_full_background(&game.scenario, &mut full_bg);

    static BACKGROUND_MUSIC: SoundData = include_wav!("sfx/game_loop.wav");

    let mut mixer = gba.mixer.mixer(Frequency::Hz18157);

    let mut background_music = SoundChannel::new(BACKGROUND_MUSIC);
    background_music.stereo();

    mixer.play_sound(background_music);

    let mut counter = 0;

    loop {
        let mut input = ButtonController::new();

        match game.state {
            GameState::Game => {
                VRAM_MANAGER.set_background_palettes(background::PALETTES);

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

                mixer.frame();
                frame.commit();
                input.update();

                // println!("{:?}", game.scenario.state);

                if input.is_just_pressed(Button::L) {
                    if game.scenario.state[3] != ScenarioType::Water {
                        game.state = GameState::GameOver;
                    }

                    game.do_action(ActionType::Attack, &mut player);
                    update_full_background(&game.scenario, &mut full_bg);
                }

                if input.is_just_pressed(Button::B) || input.is_just_pressed(Button::A) {
                    if game.scenario.state[3] != ScenarioType::Swamp {
                        game.state = GameState::GameOver;
                    }

                    game.do_action(ActionType::Shield, &mut player);
                    update_full_background(&game.scenario, &mut full_bg);
                }

                if input.is_just_pressed(Button::R) {
                    if game.scenario.state[3] != ScenarioType::Volcano {
                        game.state = GameState::GameOver;
                    }

                    game.do_action(ActionType::Jump, &mut player);
                    update_full_background(&game.scenario, &mut full_bg);
                }

                counter += 1;
            }
            GameState::GameOver => {
                VRAM_MANAGER.set_background_palette(0, PALETTE);

                let mut frame = gfx.frame();

                for object in objects.iter() {
                    object.show(&mut frame);
                }

                mixer.frame();
                frame.commit();
                input.update();

                if input.is_just_pressed(Button::START) {
                    game.scenario.randomize();
                    update_full_background(&game.scenario, &mut full_bg);
                    game.state = GameState::Game;
                }
            }
        }
    }
}
