#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::display::Priority;
use agb::display::tiled::{RegularBackground, RegularBackgroundSize, TileFormat, VRAM_MANAGER};
use agb::include_background_gfx;

include_background_gfx!(
    mod background,
    game => deduplicate "gfx/background.png",
    full_blue => deduplicate "gfx/background-full-blue.png"
);

pub fn main(mut gba: agb::Gba) -> ! {
    let mut gfx = gba.graphics.get();

    VRAM_MANAGER.set_background_palettes(background::PALETTES);

    let mut bg = RegularBackground::new(
        Priority::P3,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    let mut frame = gfx.frame();

    bg.fill_with(&background::game);
    bg.show(&mut frame);
    frame.commit();

    loop {
        agb::halt();
    }
}
