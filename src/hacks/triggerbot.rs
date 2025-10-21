use crate::game::{game::Player, state::SharedState, GAME};

static mut WAS_SHOOTING: bool = false;

pub fn triggerbot(local: *mut Player, state: &SharedState) {
    unsafe {
        if !state.triggerbot { return; }

        let local = &mut *local;
        let other = GAME.pointing_at();
        if !other.is_null() {
            let other = &mut *other;
            if other.is_enemy() {
                WAS_SHOOTING = local.attacking;
                local.attacking = true;
            }
        } else {
            if !WAS_SHOOTING {
                local.attacking = false;
            }
        }
    }
}