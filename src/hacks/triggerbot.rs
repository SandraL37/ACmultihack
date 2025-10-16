use crate::game::{game::Player, state::SharedState, GAME};



pub fn triggerbot(local: *mut Player, state: &SharedState) {
    unsafe {
        if !state.triggerbot { return; }

        let local = &mut *local;
        let other = GAME.pointing_at();
        if !other.is_null() {
            let other = &mut *other;
            if other.is_enemy() {
                local.attacking = true;
            }
        } else {
            local.attacking = false;
        }
    }
}