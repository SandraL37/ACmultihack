use std::sync::LazyLock;

use hklib::mem::{address::ptr, patch::Patch, process::Process};

use crate::player::Player;

static BASE: LazyLock<usize> = LazyLock::new(|| {
    unsafe { Process::current().base_address().unwrap() }
});

pub fn rig_weapon() {
    unsafe {
        let player = ptr::<Player>(*BASE + 0x10F4F4);
        let player = &mut *player;
        let weapon = &mut *player.current_weapon;
        let info = &mut *weapon.info;

        info.reloadtime = 0;
        info.kickback = 0;
        info.kickback = 0;
        info.spread = 0;
        info.recoil_1 = 0;
        info.recoil_2 = 0;
    }
}

pub fn infinite_ammo() {
    
    let mut patch = Patch::nop(
        *BASE + 0x637E9,
        2,
    ).unwrap();

    patch.enable().unwrap();
}