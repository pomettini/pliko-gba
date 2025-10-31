use agb::{
    display::{
        Graphics, Priority,
        font::AlignmentKind,
        tiled::{RegularBackground, RegularBackgroundSize, TileFormat, VRAM_MANAGER},
    },
    fixnum::vec2,
    include_background_gfx,
    input::{Button, ButtonController},
};
use alloc::format;

use crate::{label::Label, sfx_manager::Sfx};

include_background_gfx!(
    mod game_over_screen,
    GAME_OVER => deduplicate "gfx/game-over.png",
);

pub fn show_game_over_screen(score: usize, gfx: &mut Graphics, sfx: &mut Sfx) {
    let mut map = RegularBackground::new(
        Priority::P3,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    sfx.stop();

    let mut input = ButtonController::new();

    map.fill_with(&game_over_screen::GAME_OVER);

    VRAM_MANAGER.set_background_palettes(game_over_screen::PALETTES);

    loop {
        sfx.frame();
        input.update();

        if input.is_just_pressed(Button::START) {
            break;
        }

        let mut frame = gfx.frame();
        map.show(&mut frame);

        let mut score_label = Label::new(
            &format!("Game Over\nFinal score: {score}"),
            vec2(120 - 40, 80 - 16),
            AlignmentKind::Centre,
            18,
            80,
        );
        score_label.draw(&mut frame);

        frame.commit();
    }
}
