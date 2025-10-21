use hklib::{mem::hook::Hook, utils::errors};

use crate::game::{game::{CubeVec3, Weapon}, GAME, STATE};

static mut GATEWAY: Option<RecoilFunc> = None;
type RecoilFunc = extern "thiscall" fn(this: *mut Weapon, from: *mut CubeVec3, to: *mut CubeVec3) -> bool;
extern "thiscall" fn hkrecoil(pthis: *mut Weapon, pfrom: *mut CubeVec3, pto: *mut CubeVec3) -> bool {
    unsafe {
        let this = &*pthis;

        let active_norecoil = STATE.with(|state| { state.borrow().norecoil });
        if this.owner == GAME.local_player() && active_norecoil {
            return true;
        } else {
            GATEWAY.unwrap()(pthis, pfrom, pto)
        }
    }
}


pub fn install_hook() -> errors::Result<Hook> {
    unsafe {
        let hook = Hook::install(0x004C8BA0 as *mut u8, hkrecoil as *mut u8, 10)?;
        GATEWAY = Some(std::mem::transmute(hook.trampoline));
        Ok(hook)
    }
}