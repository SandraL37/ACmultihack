use hklib::{math::vectors::Vec3, mem::hook::Hook, utils::errors};

use crate::game::{game::{CubeVec3, Weapon}, GAME, STATE};

static mut GATEWAY: Option<ShootFunc> = None;
type ShootFunc = extern "thiscall" fn(this: *mut Weapon, dst: *mut CubeVec3) -> bool;
extern "thiscall" fn hkshoot(pthis: *mut Weapon, pdst: *mut CubeVec3) -> bool {
    unsafe {
        let this = &*pthis;
        let dst = &mut *pdst;
        let owner = &*this.owner;
        let weapon_status = &mut *this.status;
        let mag_content = &mut *this.mag_content;

        STATE.with(|state| {  
            let state = state.borrow();

            if this.owner == GAME.local_player() {
                if state.wallbang {
                    let yaw = owner.view.yaw.to_radians();
                    let pitch = owner.view.pitch.to_radians();
                    let dir = Vec3::new(
                        pitch.cos() * yaw.sin(),
                        -pitch.cos() * yaw.cos(),
                        pitch.sin(),
                    ).normalize();
        
                    let at2 = owner.head + dir.mul(1000.0);
                    dst.vec = at2;
                }
                if state.infinite_ammo && mag_content == &0 {
                    *mag_content += 1;
                }
                if state.fastshoot {
                    weapon_status.gunwait = 0;
                }
            }
        });

        GATEWAY.unwrap()(pthis, pdst)
    }
}


pub fn install_hook() -> errors::Result<Hook> {
    unsafe {
        let hook = Hook::install(0x004C7200 as *mut u8, hkshoot as *mut u8, 10)?;
        GATEWAY = Some(std::mem::transmute(hook.trampoline));
        Ok(hook)
    }
}