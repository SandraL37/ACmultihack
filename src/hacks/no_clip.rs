use crate::game::game::Player;

pub fn no_clip(plocal: *mut Player) {
    let local = unsafe { &mut *plocal };
    let state = get_no_clip_state(plocal);
    if state {
        local.player_type = 0;
    } else {
        local.player_type = 4;
    }
}

pub fn get_no_clip_state(plocal: *mut Player) -> bool {
    let local = unsafe { &mut *plocal };
    match local.player_type {
        4 => true,
        _ => false,
    }
}