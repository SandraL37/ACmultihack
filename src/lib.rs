use hklib::utils::keys::{is_key_pressed, VK_DELETE};

use crate::hooks::hookmanager::HookManager;

mod hooks;
pub mod game;
pub mod hacks;
pub mod gui;

#[hklib::dll_main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = HookManager::setup_hooks()?;
    manager.persist();

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
        if is_key_pressed(VK_DELETE) {
            break;
        }
    }

    Ok(())
}