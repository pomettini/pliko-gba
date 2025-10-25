#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use core::mem;

use agb::display::font::{AlignmentKind, Font, Layout, ObjectTextRenderer};
use agb::display::object::{Object, Size};
use agb::display::tiled::{RegularBackground, RegularBackgroundSize, TileFormat, VRAM_MANAGER};
use agb::display::{Palette16, Priority, Rgb15};
use agb::fixnum::vec2;
use agb::input::{Button, ButtonController};
use agb::sound::mixer::{Frequency, SoundChannel, SoundData};
use agb::{
    include_aseprite, include_aseprite_256, include_background_gfx, include_font, include_wav,
    println,
};
use player::*;

extern crate alloc;
use alloc::vec::Vec;

use crate::enemy::{Enemy, setup_enemies};
use crate::game_over::show_game_over_screen;
use crate::scenario::{Scenario, ScenarioType};
use crate::sfx_manager::Sfx;
use crate::title_screen::show_title_screen;

pub mod enemy;
pub mod game_over;
pub mod player;
pub mod scenario;
pub mod sfx_manager;
pub mod title_screen;

include_background_gfx!(
    mod background,
    GAME => deduplicate "gfx/background.png",
    BLUE => deduplicate "gfx/background-full-blue.png",
    RED => deduplicate "gfx/background-full-red.png",
    GREEN => deduplicate "gfx/background-full-green.png",
);

include_aseprite!(
    mod buttons,
    "gfx/buttons.aseprite",
);

pub enum ActionType {
    Attack,
    Shield,
    Jump,
}

pub fn update_full_background(scenario: &Scenario, background: &mut RegularBackground) {
    let bg = match scenario.state[3] {
        ScenarioType::Water => &background::BLUE,
        ScenarioType::Volcano => &background::RED,
        ScenarioType::Swamp => &background::GREEN,
    };
    background.fill_with(bg);
}

pub fn do_action(scenario: &mut Scenario, action: ActionType, player: &mut Player) {
    player.perform_action(action);
    scenario.next();
}

pub fn main(mut gba: agb::Gba) -> ! {
    let mut counter = 0;

    // VRAM_MANAGER.set_background_palettes(background::PALETTES);

    let mut player = Player::new();
    let mut enemies = setup_enemies();

    let mut button_left = Object::new(buttons::BLUE.sprite(0));
    button_left.set_pos((90 - 8, 135 - 7));

    let mut button_middle = Object::new(buttons::GREEN.sprite(0));
    button_middle.set_pos((111 - 8, 135 - 7));

    let mut button_right = Object::new(buttons::RED.sprite(0));
    button_right.set_pos((132 - 8, 135 - 7));

    static TITLE_MUSIC: SoundData = include_wav!("sfx/title_loop.wav");
    static GAME_MUSIC: SoundData = include_wav!("sfx/game_loop.wav");

    let mut title_music = SoundChannel::new_high_priority(TITLE_MUSIC);
    title_music.should_loop();

    let mut game_music = SoundChannel::new_high_priority(GAME_MUSIC);
    game_music.should_loop();

    let mut sfx = Sfx::create(gba.mixer.mixer(Frequency::Hz18157));

    let mut gfx = gba.graphics.get();

    show_title_screen(&mut gfx, &mut sfx);

    sfx.stop();
    sfx.play_game_theme();

    /*
    let frame = gfx.frame();
    frame.commit();
    */

    loop {
        let mut input = ButtonController::new();

        VRAM_MANAGER.set_background_palettes(background::PALETTES);

        let mut game_bg = RegularBackground::new(
            Priority::P3,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

        game_bg.fill_with(&background::GAME);

        let mut full_bg = RegularBackground::new(
            Priority::P1,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

        let mut scenario = Scenario::new();
        scenario.randomize();

        update_full_background(&scenario, &mut full_bg);

        loop {
            loop {
                let mut frame = gfx.frame();

                game_bg.show(&mut frame);
                full_bg.show(&mut frame);

                scenario.draw(&mut frame);

                if counter > 6 {
                    player.update();
                    enemies[3].update();
                    counter = 0;
                }

                player.draw(&mut frame);

                for enemy in &mut enemies {
                    enemy.draw(&mut frame);
                }

                button_left.show(&mut frame);
                button_middle.show(&mut frame);
                button_right.show(&mut frame);

                sfx.frame();
                input.update();
                frame.commit();

                // println!("{:?}", game.scenario.state);

                if input.is_just_pressed(Button::L) {
                    if scenario.state[3] != ScenarioType::Water {
                        break;
                    }

                    do_action(&mut scenario, ActionType::Attack, &mut player);
                    update_full_background(&scenario, &mut full_bg);
                }

                if input.is_just_pressed(Button::B) || input.is_just_pressed(Button::A) {
                    if scenario.state[3] != ScenarioType::Swamp {
                        break;
                    }

                    do_action(&mut scenario, ActionType::Shield, &mut player);
                    update_full_background(&scenario, &mut full_bg);
                }

                if input.is_just_pressed(Button::R) {
                    if scenario.state[3] != ScenarioType::Volcano {
                        break;
                    }

                    do_action(&mut scenario, ActionType::Jump, &mut player);
                    update_full_background(&scenario, &mut full_bg);
                }

                counter += 1;
            }

            show_game_over_screen(&mut gfx, &mut sfx);

            sfx.play_game_theme();

            VRAM_MANAGER.set_background_palettes(background::PALETTES);
        }
    }
}
