use hklib::{mem::hook::Hook, utils::errors};
// use windows::Win32::Graphics::OpenGL::*;

/*
char *a1, int a2, char a3
*/

static mut GATEWAY: Option<LoadModelFunc> = None;
type LoadModelFunc = extern "C" fn(i32);
extern "C" fn hkloadmodel(
    mdl: i32,
) {
    unsafe {
        println!("{:?}", mdl);

        GATEWAY.unwrap()(mdl);
    }
}

pub fn install_hook() -> errors::Result<Hook> {
    unsafe {
        let hook = Hook::install(0x004658F0 as *mut u8, hkloadmodel as *mut u8, 6)?;
        GATEWAY = Some(std::mem::transmute(hook.trampoline));
        Ok(hook)
    }
}
