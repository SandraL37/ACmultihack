use hklib::{mem::hook::Hook, utils::errors};

use crate::hooks::*;

pub struct HookManager {
    pub _shoot: Hook,
    pub _wgl_swap_buffers: Hook,
    pub _recoil: Hook,
    pub _render_clients: Hook,
    // pub _render_model: Hook,
    // pub _load_model: Hook,
}

impl HookManager {
    pub fn setup_hooks() -> errors::Result<Self> {
        Ok(HookManager {
            _shoot: shoot::install_hook()?,
            _wgl_swap_buffers: wglswapbuffers::install_hook()?,
            _recoil: recoil::install_hook()?,
            _render_clients: renderclients::install_hook()?,
            // _render_model: rendermodel::install_hook()?,
            // _load_model: loadmodel::install_hook()?,
        })
    }
}