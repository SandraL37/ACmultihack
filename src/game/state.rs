use std::cell::RefMut;

use hklib::mem::patch::Patch;

pub struct State {
    pub aimbot: bool,
    pub crosshair: bool,
    pub esp: bool,
    pub no_clip: bool,
    pub trace: bool,
    pub triggerbot: bool,
    pub wallbang: bool,
    pub show_menu: bool,
    pub maphack: Patch,
}

pub type SharedState<'a> = RefMut<'a, State>;