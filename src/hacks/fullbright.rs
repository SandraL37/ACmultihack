use crate::game::GAME;

pub struct Fullbright {
    enabled: bool,
}

impl Fullbright {
    pub fn new() -> Self {
        Self { enabled: false }
    }

    pub fn toggle(&mut self) {
        if self.enabled {
            reset_lights();
        } else {
            set_lights(255, 255, 255);
        }
        self.enabled = !self.enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

pub fn reset_lights() {
    let _reset_lights: extern "C" fn() = unsafe { std::mem::transmute(0x004BA180) };
    _reset_lights();
}

pub fn set_lights(r: u8, g: u8, b: u8) {
    for light in GAME.lights() {
        let light = unsafe { &mut *light };
        light.r = r;
        light.g = g;
        light.b = b;
    }
}

pub fn fullbright() -> Fullbright {
    Fullbright::new()
}