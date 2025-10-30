#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::display::Priority;
use agb::display::font::AlignmentKind;
use agb::display::tiled::RegularBackgroundSize;
use agb::display::tiled::{RegularBackground, TileFormat, VRAM_MANAGER};
use agb::input::ButtonController;
use agb::interrupt::VBlank;
use agb::rng::RandomNumberGenerator;
use agb::sound::mixer::Frequency;
use agb::{fixnum, include_background_gfx};
use alloc::format;
use fixnum::vec2;
use player::*;

extern crate alloc;

use crate::binding::{ActionType, BINDINGS};
use crate::buttons::get_buttons;
use crate::countdown::Countdown;
use crate::enemy::setup_enemies;
use crate::game_over::show_game_over_screen;
use crate::label::Label;
use crate::scenario::{Scenario, ScenarioType};
use crate::sfx_manager::Sfx;
use crate::title_screen::show_title_screen;

pub mod binding;
pub mod buttons;
pub mod countdown;
pub mod enemy;
pub mod game_over;
pub mod label;
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
    let mut rng = RandomNumberGenerator::new();

    let mut player = Player::new();
    let mut enemies = setup_enemies();
    let mut buttons = get_buttons();

    let mut sfx = Sfx::create(gba.mixer.mixer(Frequency::Hz18157));
    let mut gfx = gba.graphics.get();

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

        let mut scenario = Scenario::new(rng);
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

                let mut score_label = Label::new(
                    &format!("Score: {enemies_killed}"),
                    vec2(8, 3),
                    AlignmentKind::Left,
                    18,
                    80,
                );
                let mut time_label = Label::new(
                    &format!("Time: {0}", countdown.seconds_left()),
                    vec2(0, 3),
                    AlignmentKind::Right,
                    16,
                    232,
                );

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

                score_label.draw(&mut frame);
                time_label.draw(&mut frame);

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
                } else {
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
