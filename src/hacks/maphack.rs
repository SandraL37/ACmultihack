use hklib::mem::patch::Patch;

pub fn maphack() -> Patch {
    let addr = 0x0045D2C4;
    let patch = Patch::new(addr, vec![0x0f, 0x85]).unwrap();
    patch
}