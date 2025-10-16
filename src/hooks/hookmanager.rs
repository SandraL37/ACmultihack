use hklib::{mem::hook::Hook, utils::errors};

use crate::hooks::{shoot, wglswapbuffers};

pub struct HookManager {
    pub _shoot: Hook,
    pub _wgl_swap_buffers: Hook,
}

impl HookManager {
    pub fn setup_hooks() -> errors::Result<Self> {
        let hkshoot = shoot::install_hook()?;
        let hkwglswapbuffers = wglswapbuffers::install_hook()?;
        Ok(HookManager {
            _shoot: hkshoot,
            _wgl_swap_buffers: hkwglswapbuffers,
        })
    }

    pub const fn persist(&self) {
        // Call this placebo function to make rust drop the hooks only when the program exits
    }
}