use hklib::mem::patch::Patch;

pub struct WallHack {
    enabled: bool,
    occlusion: Patch,
}

impl WallHack {
    pub fn new() -> Self {
        Self { 
            enabled: false,
            occlusion: patch_occlusion(),
        }
    }

    pub fn toggle(&mut self) {
        self.occlusion.toggle().unwrap();
        self.enabled = !self.enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

fn patch_occlusion() -> Patch {
    Patch::nop(0x00463BDF, 6).unwrap()
}

pub fn wallhack() -> WallHack {
    WallHack::new()
}