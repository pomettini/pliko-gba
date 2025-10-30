use agb::{
    display::{
        GraphicsFrame, Palette16, Rgb15,
        font::{AlignmentKind, Font, Layout, ObjectTextRenderer},
        object::{Object, Size},
    },
    fixnum::Vector2D,
    include_font,
};
use alloc::{string::String, vec::Vec};

pub static PALETTE: &Palette16 = const {
    let mut palette = [Rgb15::BLACK; 16];
    palette[1] = Rgb15::WHITE;
    palette[2] = Rgb15(0x10_7C);
    &Palette16::new(palette)
};

static FONT: Font = include_font!("gfx/ark-pixel-10px-proportional.ttf", 10);

pub struct Label {
    objects: Vec<Object>,
}

impl Label {
    pub fn new(
        text: &String,
        offset: Vector2D<i32>,
        alignment: AlignmentKind,
        max_group_width: i32,
        max_line_length: i32,
    ) -> Self {
        let score_layout = Layout::new(text, &FONT, alignment, max_group_width, max_line_length);
        let text_render = ObjectTextRenderer::new(PALETTE.into(), Size::S16x16);
        let objects = score_layout.map(|x| text_render.show(&x, offset)).collect();
        Self { objects }
    }

    pub fn draw(&mut self, frame: &mut GraphicsFrame<'_>) {
        for object in self.objects.iter() {
            object.show(frame);
        }
    }
}
