use hklib::mem::patch::Patch;

pub struct Maphack {
    pub enabled: bool,
    pub patch_big: Patch,
    pub patch_small: Patch,
}

impl Maphack {
    pub fn toggle(&mut self) {
        self.patch_big.toggle().unwrap();
        self.patch_small.toggle().unwrap();
    }
    pub fn is_enabled(&self) -> bool {
        self.patch_big.is_enabled() && self.patch_small.is_enabled()
    }
}

pub fn maphack() -> Maphack {
    let patch_big = Patch::new(0x0045D2C4, vec![0x0f, 0x85]).unwrap();
    let patch_small = Patch::new(0x0045C4D6, vec![0x0f, 0x85]).unwrap();

    Maphack {
        enabled: true,
        patch_big,
        patch_small,
    }
}