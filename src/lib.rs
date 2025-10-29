#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::display::font::{AlignmentKind, Font, Layout, ObjectTextRenderer};
use agb::display::object::{Object, Size};
use agb::display::tiled::{RegularBackground, RegularBackgroundSize, TileFormat, VRAM_MANAGER};
use agb::display::{Palette16, Priority, Rgb15};
use agb::input::{Button, ButtonController};
use agb::interrupt::VBlank;
use agb::sound::mixer::Frequency;
use agb::{fixnum, include_aseprite, include_background_gfx, include_font};
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::vec::Vec;
use fixnum::vec2;
use player::*;

extern crate alloc;

use crate::countdown::Countdown;
use crate::enemy::setup_enemies;
use crate::game_over::show_game_over_screen;
use crate::scenario::{Scenario, ScenarioType};
use crate::sfx_manager::Sfx;
use crate::title_screen::show_title_screen;

pub mod countdown;
pub mod enemy;
pub mod game_over;
pub mod player;
pub mod scenario;
pub mod sfx_manager;
pub mod title_screen;

static PALETTE: &Palette16 = const {
    let mut palette = [Rgb15::BLACK; 16];
    palette[1] = Rgb15::WHITE;
    palette[2] = Rgb15(0x10_7C);
    &Palette16::new(palette)
};

static FONT: Font = include_font!("gfx/ark-pixel-10px-proportional.ttf", 10);

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

#[derive(Clone, Copy)]
pub enum ActionType {
    Attack,
    Shield,
    Jump,
}

#[derive(Clone, Copy)]
pub struct Binding {
    button: Button,
    scenario: ScenarioType,
    action: ActionType,
}

const BINDINGS: &[Binding] = &[
    Binding {
        button: Button::L,
        scenario: ScenarioType::Water,
        action: ActionType::Attack,
    },
    Binding {
        button: Button::R,
        scenario: ScenarioType::Volcano,
        action: ActionType::Jump,
    },
    Binding {
        button: Button::B,
        scenario: ScenarioType::Swamp,
        action: ActionType::Shield,
    },
    Binding {
        button: Button::A,
        scenario: ScenarioType::Swamp,
        action: ActionType::Shield,
    },
];

pub fn update_full_background(scenario: &Scenario, background: &mut RegularBackground) {
    let bg = match scenario.state[3] {
        ScenarioType::Water => &background::BLUE,
        ScenarioType::Volcano => &background::RED,
        ScenarioType::Swamp => &background::GREEN,
    };
    background.fill_with(bg);
}

pub fn check_game_over(
    scenario: &Scenario,
    scenario_type: ScenarioType,
    player: &mut Player,
) -> bool {
    let wrong_action = scenario.state[3] != scenario_type;
    if wrong_action {
        player.kill();
    }
    wrong_action
}

pub fn do_action(scenario: &mut Scenario, action: ActionType, player: &mut Player) {
    player.perform_action(action);
    scenario.next();
}

pub fn main(mut gba: agb::Gba) -> ! {
    let mut player = Player::new();
    let mut enemies = setup_enemies();

    let mut buttons: [Object; 3] = [
        Object::new(buttons::BLUE.sprite(0))
            .set_pos((90 - 8, 135 - 7))
            .to_owned(),
        Object::new(buttons::GREEN.sprite(0))
            .set_pos((111 - 8, 135 - 7))
            .to_owned(),
        Object::new(buttons::RED.sprite(0))
            .set_pos((132 - 8, 135 - 7))
            .to_owned(),
    ];

    let mut sfx = Sfx::create(gba.mixer.mixer(Frequency::Hz18157));
    let mut gfx = gba.graphics.get();

    let score_text_render = ObjectTextRenderer::new(PALETTE.into(), Size::S16x16);
    let time_text_render = ObjectTextRenderer::new(PALETTE.into(), Size::S16x16);

    show_title_screen(&mut gfx, &mut sfx);

    sfx.stop();
    sfx.play_game_theme();

    let mut enemies_killed: usize;
    let mut death_counter: usize;

    // Game setup
    loop {
        let vblank = VBlank::get();

        let mut timers = gba.timers.timers();
        let mut countdown = Countdown::new(10, &mut timers.timer2, &mut timers.timer3);
        let mut input = ButtonController::new();

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

        // Game restart
        loop {
            VRAM_MANAGER.set_background_palettes(background::PALETTES);

            enemies_killed = 0;
            death_counter = 0;

            player.reset();
            countdown.reset();
            countdown.set_enabled(true);

            // Game update
            loop {
                vblank.wait_for_vblank();

                sfx.frame();
                countdown.update();

                if countdown.seconds_left() <= 0 {
                    player.kill();
                }

                player.update();
                enemies[3].update();

                let score_layout = Layout::new(
                    &format!("Score: {enemies_killed}"),
                    &FONT,
                    AlignmentKind::Left,
                    16,
                    80,
                );

                let score_objects: Vec<_> = score_layout
                    .map(|x| score_text_render.show(&x, vec2(8, 3)))
                    .collect();

                let time_layout = Layout::new(
                    &format!("Time: {0}", countdown.seconds_left()),
                    &FONT,
                    AlignmentKind::Right,
                    16,
                    232,
                );

                let time_objects: Vec<_> = time_layout
                    .map(|x| time_text_render.show(&x, vec2(0, 3)))
                    .collect();

                let mut frame = gfx.frame();

                player.draw(&mut frame);

                for enemy in &mut enemies {
                    enemy.draw(&mut frame);
                }

                for button in &mut buttons {
                    button.show(&mut frame);
                }

                game_bg.show(&mut frame);
                full_bg.show(&mut frame);

                scenario.draw(&mut frame);

                for object in score_objects.iter() {
                    object.show(&mut frame);
                }

                for object in time_objects.iter() {
                    object.show(&mut frame);
                }

                input.update();
                frame.commit();

                if !player.is_dead() {
                    for binding in BINDINGS {
                        if input.is_just_pressed(binding.button)
                            && !check_game_over(&scenario, binding.scenario, &mut player)
                        {
                            do_action(&mut scenario, binding.action, &mut player);
                            update_full_background(&scenario, &mut full_bg);
                            enemies_killed += 1;
                        }
                    }
                }

                if player.is_dead() {
                    countdown.set_enabled(false);
                    if death_counter > 50 {
                        break;
                    }
                    death_counter += 1;
                }
            }

            show_game_over_screen(&mut gfx, &mut sfx);

            sfx.play_game_theme();
        }
    }
}
