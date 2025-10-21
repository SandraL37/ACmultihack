
use hklib::{mem::hook::Hook, utils::errors};
use windows::Win32::Graphics::OpenGL::*;

use crate::game::{game::Player, GAME, STATE};

// static mut GATEWAY: Option<RenderClientsFunc> = None;
// type RenderClientsFunc = extern "C" fn();
extern "C" fn hkrenderclients() {
    unsafe {
        let renderclient: extern "thiscall" fn(*mut Player) = std::mem::transmute(0x00462520);
        STATE.with(|state| {
            let state = state.borrow();
            
            for player in GAME.entity_list() {
                if state.wallhack.is_enabled() {
                    // glBindTexture(GL_TEXTURE_2D, 0);
					// glEnable(GL_DEPTH_TEST);
					// glDepthFunc(GL_LESS);
					// glEnable(GL_BLEND);
					// glBlendFunc(GL_SRC_COLOR, GL_SRC_COLOR); 
                    
					// glColor4f(0.4, 1.0, 0.2, 0.1);
                    
                    
                    // Render the player model here
                    
                    // glDepthRange(0.0, 1.0);
                    // glDepthFunc(GL_ALWAYS);
                    glDisable(GL_DEPTH_TEST);
                    // CHAMS
                    glDisable(GL_TEXTURE_2D);
                    // disable light impact on texture
                    glDisable(GL_LIGHTING);
                    // disable all light
                    glDisable(GL_LIGHT0);
                    glDisable(GL_LIGHT1);
                    glDisable(GL_LIGHT2);
                    glDisable(GL_LIGHT3);
                    glDisable(GL_LIGHT4);
                    glDisable(GL_LIGHT5);
                    glDisable(GL_LIGHT6);
                    glDisable(GL_LIGHT7);
                    // set texture color
                    glColor4f(0.0, 1.0, 0.0, 1.0);
                    
                }
                renderclient(player);
            }
        });
    }
}

pub fn install_hook() -> errors::Result<Hook> {
    unsafe {
        let hook = Hook::install(0x00467EE0 as *mut u8, hkrenderclients as *mut u8, 7)?;
        // GATEWAY = Some(std::mem::transmute(hook.trampoline));
        Ok(hook)
    }
}
