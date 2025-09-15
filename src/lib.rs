use hklib::mem::{hook::Hook, process::Process};
use windows::Win32::Graphics::Gdi::HDC;

static mut GATEWAY: Option<*mut u8> = None;

extern "stdcall" fn hkwgl_swap_buffers(hdc: HDC) {

    println!("{:#?}", hdc);

    unsafe {
        if let Some(gateway) = GATEWAY {
            let gateway: extern "stdcall" fn(HDC) -> i32 = std::mem::transmute(gateway);
            gateway(hdc);
            return;
        }
    }
}

#[hklib::dll_main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let wgl_swap_buffers = Process::current()
            .get_module("opengl32.dll")?
            .get_symbol("wglSwapBuffers").unwrap() as *mut u8;

        let mut hook = Hook::new(wgl_swap_buffers, hkwgl_swap_buffers as *mut u8, 5)?;
        hook.enable()?;
        GATEWAY = Some(hook.trampoline);

        std::io::stdin().read_line(&mut String::new()).unwrap();
    }

    std::thread::sleep(std::time::Duration::from_millis(100));
    Ok(())
}