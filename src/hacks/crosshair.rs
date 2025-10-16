use hklib::graphics::{
    common::{drawable::circle::Circle, options::color::Color},
    opengl::Overlay,
};

use crate::game::{state::SharedState, GAME};

pub fn crosshair(overlay: &Overlay, state: &SharedState) {
    unsafe {
        if !state.crosshair { return; }

        overlay
            .draw_or_err(&Circle::new33(
                *GAME.pointed_position(),
                0.1,
                64,
                0.05,
                Color::CYAN,
                &overlay,
            ))
            .ok();
    }
}
