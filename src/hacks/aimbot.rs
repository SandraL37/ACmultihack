use crate::game::{game::Player, state::SharedState};

pub fn get_best_target(
    local: *mut Player,
    pother: *mut Player,
    target: &mut Option<*mut Player>,
    state: &SharedState,
) {
    let local = unsafe { &*local };
    let other = unsafe { &*pother };

    if other.health <= 0 || !other.is_enemy() {
        return;
    }

    if !(state.wallbang || other.is_visible()) {
        return;
    }

    match target {
        Some(t) => {
            let t = unsafe { &**t };

            let angle = local.head.angle_to(&other.head);
            let target_angle = local.head.angle_to(&t.head);

            let d_angle = angle - local.view;
            let d_target_angle = target_angle - local.view;

            if d_angle < d_target_angle {
                *target = Some(pother);
            }
        }
        None => {
            *target = Some(pother);
        }
    }
}

pub fn aimbot(local: *mut Player, ptarget: Option<*mut Player>, state: &SharedState) {
    if !state.aimbot {
        return;
    }

    unsafe {
        let local = &mut *local;

        if let Some(target) = ptarget {
            let target = &*target;

            local.view = local.head.angle_to(&target.head);
        }
    }
}
