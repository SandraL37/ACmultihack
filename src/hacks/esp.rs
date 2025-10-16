use hklib::{
    graphics::{
        common::{
            drawable::{
                line::Line,
                rect::{Rect, RectType},
            },
            options::{alignment::Alignment, color::Color},
        },
        opengl::Overlay,
    },
    math::vectors::Vec2,
};

use crate::game::{game::Player, state::{SharedState}};

pub fn esp(plocal: *mut Player, pother: *mut Player, overlay: &Overlay, state: &SharedState) {
    let _local = unsafe { &*plocal };
    let other = unsafe { &*pother };

    if other.is_dead() {
        return;
    }

    let rect = Rect::new3(
        other.head,
        3.0,
        6.90,
        Alignment::Center,
        Alignment::Start,
        if other.is_visible() {
            RectType::Outline(1.0)
        } else {
            RectType::TransparentOutline(64, 1.0)
        },
        match other.is_enemy() {
            true => Color::RED,
            false => Color::GREEN,
        },
        &overlay,
    );

    if let Some(rect) = rect {
        if state.esp {
            let health_bar = Line::from_length_angle(
                rect.top_left(),
                rect.width * (other.health as f32 / 100.0),
                0.0,
                1.0,
                Color::WHITE,
            );
            if overlay.distance_from_camera(&other.head) < 50.0 {
                overlay.text(
                    rect.top_left().sub(0.0, 10.0),
                    format!("{}", other.name()).as_str(),
                    Color::WHITE,
                );
                overlay.text(
                    rect.bottom_left().add(0.0, 10.0),
                    format!("Ammo {}", unsafe {
                        *other.current_weapon.as_ref().unwrap().ammo
                    })
                    .as_str(),
                    Color::WHITE,
                );
            }
        
            overlay.draw(&rect);
            overlay.draw(&health_bar);
        }
        
        if state.trace {
            trace(plocal, pother, &rect, overlay);
        }
    }
}

fn trace(local: *mut Player, other: *mut Player, rect: &Rect, overlay: &Overlay) {
    let _local = unsafe { &*local };
    let other = unsafe { &*other };

    if other.is_dead() {
        return;
    };

    let screen = Overlay::window();

    let _ = overlay.draw(&Line::new(
        Vec2::new(screen.width as f32 / 2.0, screen.height),
        Vec2::new(rect.center().x, rect.bounds.bottom),
        0.5,
        match other.is_enemy() {
            true => Color::RED,
            false => Color::GREEN,
        },
    ));
}
