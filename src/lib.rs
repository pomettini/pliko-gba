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
use agb::sound::mixer::{Frequency, SoundChannel, SoundData};
use agb::timer::{Divider, Timer};
use agb::{
    Gba, fixnum, include_aseprite, include_background_gfx, include_font, include_wav, println,
};
use alloc::format;
use alloc::vec::Vec;
use fixnum::vec2;
use player::*;

extern crate alloc;

use crate::enemy::setup_enemies;
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

pub fn check_game_over(
    scenario: &Scenario,
    scenario_type: ScenarioType,
    player: &mut Player,
) -> bool {
    let wrong_action = scenario.state[3] != scenario_type;

    if wrong_action {
        player.kill();
    }

    return wrong_action;
}

pub fn do_action(scenario: &mut Scenario, action: ActionType, player: &mut Player) {
    player.perform_action(action);
    scenario.next();
}

pub fn main(mut gba: agb::Gba) -> ! {
    let mut counter = 0;

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

    let score_text_render = ObjectTextRenderer::new(PALETTE.into(), Size::S16x16);
    let time_text_render = ObjectTextRenderer::new(PALETTE.into(), Size::S16x16);

    show_title_screen(&mut gfx, &mut sfx);

    sfx.stop();
    sfx.play_game_theme();

    // Game setup
    loop {
        let vblank = VBlank::get();

        let timers = gba.timers.timers();
        let mut t2 = timers.timer2;
        let mut t3 = timers.timer3;

        t2.set_divider(Divider::Divider1024).set_enabled(true);
        t3.set_cascade(true).set_enabled(true);

        let mut last_ticks: u32 = 0;
        let mut seconds_left: i32 = 60;

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

            player.state = PlayerState::Idle;

            // Game update
            loop {
                vblank.wait_for_vblank();

                for _ in 0..200 {
                    sfx.frame();
                }

                let low = t2.value() as u32;
                let high = t3.value() as u32;
                let ticks = (high << 16) | low;

                let delta = ticks.wrapping_sub(last_ticks);
                last_ticks = ticks;

                static mut ACC: u32 = 0;
                unsafe {
                    ACC += delta;
                    if ACC >= 16384 {
                        ACC -= 16384;
                        if seconds_left > 0 {
                            seconds_left -= 1;
                        }
                    }
                }

                player.update();
                if counter > 6 {
                    enemies[3].update();
                    counter = 0;
                }

                let score_layout =
                    Layout::new(&format!("Score: 0"), &FONT, AlignmentKind::Left, 16, 80);

                let score_objects: Vec<_> = score_layout
                    .map(|x| score_text_render.show(&x, vec2(8, 3)))
                    .collect();

                let time_layout = Layout::new(
                    &format!("Time: {seconds_left}"),
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

                button_left.show(&mut frame);
                button_middle.show(&mut frame);
                button_right.show(&mut frame);

                game_bg.show(&mut frame);
                full_bg.show(&mut frame);

                scenario.draw(&mut frame);

                for object in score_objects.iter() {
                    object.show(&mut frame);
                }

                for object in time_objects.iter() {
                    object.show(&mut frame);
                }

                // sfx.frame();
                input.update();
                frame.commit();

                if input.is_just_pressed(Button::L) {
                    if !check_game_over(&scenario, ScenarioType::Water, &mut player) {
                        do_action(&mut scenario, ActionType::Attack, &mut player);
                        update_full_background(&scenario, &mut full_bg);
                    }
                }

                if input.is_just_pressed(Button::B) || input.is_just_pressed(Button::A) {
                    if !check_game_over(&scenario, ScenarioType::Swamp, &mut player) {
                        do_action(&mut scenario, ActionType::Shield, &mut player);
                        update_full_background(&scenario, &mut full_bg);
                    }
                }

                if input.is_just_pressed(Button::R) {
                    if !check_game_over(&scenario, ScenarioType::Volcano, &mut player) {
                        do_action(&mut scenario, ActionType::Jump, &mut player);
                        update_full_background(&scenario, &mut full_bg);
                    }
                }

                if player.is_dead() {
                    break;
                }

                counter += 1;
            }

            show_game_over_screen(&mut gfx, &mut sfx);

            sfx.play_game_theme();
        }
    }
}
