use hklib::{
    graphics::opengl::Overlay, mem::{hook::Hook, process::Process}, utils::{errors, keys::*}
};

use crate::{game::{GAME, STATE}, gui::GUI, hacks};

static mut GATEWAY: Option<extern "system" fn(*mut std::ffi::c_void) -> i32> = None;
extern "system" fn hkwgl_swap_buffers(hdc: *mut std::ffi::c_void) -> i32 {
    unsafe {
        let local = GAME.local_player();
        
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.no_clip = hacks::get_no_clip_state(local);
    
            if is_key_pressed(VK_H) {
                state.show_menu = !state.show_menu;
            }
            if is_key_pressed(VK_F1) {
                state.triggerbot = !state.triggerbot;
            }
            if is_key_pressed(VK_F2) {
                state.esp = !state.esp;
            }
            if is_key_pressed(VK_F3) {
                state.trace = !state.trace;
            }
            if is_key_pressed(VK_F4) {
                state.aimbot = !state.aimbot;
            }
            if is_key_pressed(VK_C) {
                state.no_clip = !state.no_clip;
                hacks::no_clip(local);
            }
            if is_key_pressed(VK_F6) {
                state.crosshair = !state.crosshair;
            }
            if is_key_pressed(VK_F7) {
                state.wallbang = !state.wallbang;
            }
            if is_key_pressed(VK_F8) {
                state.maphack.toggle();
            }
            if is_key_pressed(VK_F9) {
                state.norecoil = !state.norecoil;
            }
            if is_key_pressed(VK_NUMPAD0) {
                state.infinite_ammo = !state.infinite_ammo;
            }
            if is_key_pressed(VK_NUMPAD1) {
                state.fastshoot = !state.fastshoot;
            }
            if is_key_pressed(VK_NUMPAD2) {
                state.fullbright.toggle();
            }
            if is_key_pressed(VK_NUMPAD3) {
                state.wallhack.toggle();
            }
    
            let overlay = Overlay::new(GAME.view_matrix(), 90.0).unwrap();
            let mut target = None;
    
            for other in GAME.entity_list() {
                hacks::esp(local, other, &overlay, &state);
    
                hacks::get_best_target(local, other, &mut target, &state);
            }
            
            hacks::triggerbot(local, &state);
    
            if is_key_down(VK_LCONTROL) {
                hacks::aimbot(local, target, &state);
            }
    
            hacks::crosshair(&overlay, &state);
            
            if state.show_menu {
    
                GUI.with(|gui| {
                    gui.borrow_mut().draw(&mut state);
                });
            }
        });

        GATEWAY.unwrap()(hdc)
    }
}

pub fn install_hook() -> errors::Result<Hook> {
    unsafe {
        let wgl_swap_buffers = Process::current()
            .get_module("opengl32.dll")?
            .get_symbol("wglSwapBuffers")
            .unwrap() as *mut u8;
        let hook = Hook::install(wgl_swap_buffers, hkwgl_swap_buffers as *mut u8, 5)?;
        GATEWAY = Some(std::mem::transmute(hook.trampoline));
        Ok(hook)
    }
}
