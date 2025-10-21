use hklib::utils::keys::{is_key_pressed, VK_DELETE};

use crate::hooks::hookmanager::HookManager;

mod hooks;
pub mod game;
pub mod hacks;
pub mod gui;

#[hklib::dll_main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _manager = HookManager::setup_hooks()?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(10));
        if is_key_pressed(VK_DELETE) {
            break;
        }
    }

    Ok(())
}