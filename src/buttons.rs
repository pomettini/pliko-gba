use agb::{display::object::Object, include_aseprite};
use alloc::borrow::ToOwned;

include_aseprite!(
    mod buttons,
    "gfx/buttons.aseprite",
);

pub fn get_buttons() -> [Object; 3] {
    [
        Object::new(buttons::BLUE.sprite(0))
            .set_pos((90 - 8, 135 - 7))
            .to_owned(),
        Object::new(buttons::GREEN.sprite(0))
            .set_pos((111 - 8, 135 - 7))
            .to_owned(),
        Object::new(buttons::RED.sprite(0))
            .set_pos((132 - 8, 135 - 7))
            .to_owned(),
    ]
}
