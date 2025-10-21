use std::cell::RefMut;
use crate::hacks::{Fullbright, Maphack, WallHack};

pub struct State {
    pub aimbot: bool,
    pub crosshair: bool,
    pub esp: bool,
    pub no_clip: bool,
    pub trace: bool,
    pub triggerbot: bool,
    pub wallbang: bool,
    pub maphack: Maphack,
    pub infinite_ammo: bool,
    pub norecoil: bool,
    pub fastshoot: bool,
    pub fullbright: Fullbright,
    pub wallhack: WallHack,
    pub show_menu: bool,
}

pub type SharedState<'a> = RefMut<'a, State>;