use hklib::{mem::hook::Hook, utils::errors};
// use windows::Win32::Graphics::OpenGL::*;


/*
d rendermodel(const char *mdl, int anim, int tex, float rad, const vec &o, float roll, float yaw, float pitch, float speed, int basetime, playerent *d, modelattach *a, float scale)
*/
static mut GATEWAY: Option<RenderModelFunc> = None;
type RenderModelFunc = extern "C" fn(i32, i32, i32, f32, i32, f32, f32, f32, f32, i32, u32, u32, f32);
extern "C" fn hkrendermodel(
    mdl: i32,
    anim: i32,
    tex: i32,
    rad: f32,
    o: i32,
    roll: f32,
    yaw: f32,
    pitch: f32,
    speed: f32,
    basetime: i32,
    d: u32,
    a: u32,
    scale: f32,
) {
    unsafe {
        println!("{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}", mdl, anim, tex, rad, o, roll, yaw, pitch, speed, basetime, d, a, scale);

        GATEWAY.unwrap()(mdl, anim, tex, rad, o, roll, yaw, pitch, speed, basetime, d, a, scale);
    }
}

pub fn install_hook() -> errors::Result<Hook> {
    unsafe {
        let hook = Hook::install(0x00463AD0 as *mut u8, hkrendermodel as *mut u8, 10)?;
        GATEWAY = Some(std::mem::transmute(hook.trampoline));
        Ok(hook)
    }
}
