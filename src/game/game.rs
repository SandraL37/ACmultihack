use std::ffi::CStr;

use hklib::{
    math::{angles::Angle2, vectors::Vec3}, mem::process::Process, utils::errors::Result
};

use crate::game::GAME;

pub mod offsets {
    pub const LOCALPLAYER: usize = 0x18AC00;
    pub const ENTITYLIST: usize = 0x18AC04;
    pub const ENTITYCOUNT: usize = 0x18AC0C;
    pub const VIEWMATRIX: usize = 0x17DFD0;
    pub const CURRENT_FRAME: usize = 0x17F10C;
	pub const GAMEMODE: usize = 0x18ABF8;
	pub const POINTED_POSITION: usize = 0x00192064;
    // pub const EDITING_ACTIVE: usize = 0x001801BD;
    // pub const FULLBRIGHT_ACTIVE: usize = 0x001801C1;
    pub const LIGHTS: usize = 0x00182938;
    pub const LIGHTS_BYTES: usize = 0x00182998;
}

pub struct Game {
    pub base: usize,
    pub exit: bool,
}

impl Game {
    pub fn new() -> Result<Self> {
        unsafe {
            let process = Process::current().base_address()?;
            Ok(Self {
                base: process,
				exit: false,
            })
        }
    }

    fn ptr_at<T>(&self, offset: usize) -> *mut T {
        unsafe { *((self.base + offset) as *const usize) as *mut T }
    }

    pub fn local_player(&self) -> *mut Player {
        self.ptr_at(offsets::LOCALPLAYER)
    }

	pub fn entity_count(&self) -> usize {
		unsafe { *((self.base + offsets::ENTITYCOUNT) as *const usize) }
	}

	pub fn entity_list(&self) -> Vec<*mut Player> {
		unsafe {
			let current = self.ptr_at::<*mut Player>(offsets::ENTITYLIST);
			let mut entities = Vec::new();
	
			if current.is_null() {
				return entities;
			}

			for i in 1..self.entity_count() {
				entities.push(*current.add(i))
			}
			entities
		}
	}

	pub fn view_matrix(&self) -> *const [f32; 16] {
		(self.base + offsets::VIEWMATRIX) as *const [f32; 16]
	}

	pub fn pointed_position(&self) -> *const Vec3 {
		(self.base + offsets::POINTED_POSITION) as *const Vec3
	}

	pub fn current_frame(&self) -> usize {
		unsafe { *((self.base + offsets::CURRENT_FRAME) as *const usize) }
	}

	pub fn is_team_game(&self) -> bool {
		match unsafe { *((self.base + offsets::GAMEMODE) as *mut usize) } {
			7 | 20 | 21 => true,
			_ => false,
		}
	}

	pub fn pointing_at(&self) -> *mut Player {
		let pointed_player: extern "C" fn() -> *mut Player = unsafe { std::mem::transmute(0x004CC890) };
		pointed_player()
	}

	pub fn print(&self, msg: String) {
		let console: extern "C" fn(usize, usize) = unsafe { std::mem::transmute(0x004DAD50) };
		let binding = msg + "\0";
  	    let c_str = CStr::from_bytes_with_nul(binding.as_bytes()).unwrap();
		console(c_str.as_ptr() as usize, 0);
	}

    pub fn lights(&self) -> Vec<*mut Light> {
        let base = self.ptr_at::<Light>(offsets::LIGHTS);
        let mut lights = Vec::new();
        for i in 0..self.lights_count() {
            let light = unsafe { base.add(i as usize) };
            lights.push(light);
        }
        lights
    }

    pub fn lights_count(&self) -> u32 {
        unsafe { *((self.base + offsets::LIGHTS_BYTES) as *mut u32) }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Light {
    pub n00000530: u8, //0x0000
    pub n00000541: u8, //0x0001
    pub smt1: u8, //0x0002
    pub smt2: u8, //0x0003
    pub smt3: u8, //0x0004
    pub smt4: u8, //0x0005
    pub r: u8, //0x0006
    pub g: u8, //0x0007
    pub b: u8, //0x0008
    pub n00000547: u8, //0x0009
    pub n00000553: u8, //0x000A
    pub n00000548: u8, //0x000B
    pub smt5: u8, //0x000C
    pub n0000054a: u8, //0x000D
    pub n00000556: u8, //0x000E
    pub n0000054b: u8, //0x000F
} //Size: 0x0010

#[repr(C)]
#[derive(Clone)]
pub struct Player {
	pub pad_0000: [u8; 4],                       //0x0000
    pub head: Vec3,                              // 0x0004
    pad_0010: [u8; 24],                          // 0x0010
    pub position: Vec3,                          // 0x0028
    pub view: Angle2,                            // 0x0034
    pad_003c: [u8; 16],                          // 0x003C
    pub intersectflt1: f32,                      // 0x004C
    pub intersectflt2: f32,                      // 0x0050
    pub intersectflt3: f32,                      // 0x0054
    pub intersectflt4: f32,                      // 0x0058
    pad_005c: [u8; 8],                           // 0x005C
    pub p000001: u16,                            // 0x0064
    pub scooping: u8,                            // 0x0066
    pad_0067: [u8; 13],                          // 0x0067
    pub n0000006b: u16,                          // 0x0074
    pub player_type: u8,                         // 0x0076
    pad_0077: [u8; 93],                          // 0x0077
    pub n00000082: u32,                          // 0x00D4
    pub n00000084: u32,                          // 0x00D8
    pad_00dc: [u8; 8],                           // 0x00DC
    pub last_visible_frame: usize,               // 0x00E4
    pad_00e8: [u8; 4],                           // 0x00E8
    pub health: i32,                             // 0x00EC
    pub armour: u32,                             // 0x00F0
    pad_00f4: [u8; 248],                         // 0x00F4
    pub last_action: u32,                        // 0x01EC
    pad_01f0: [u8; 20],                          // 0x01F0
    pub attacking: bool,                         // 0x0204
    pub name: [u8; 15],                          // 0x0205
    pad_0214: [u8; 248],                         // 0x0214
    pub team: u8,                                // 0x030C
    pad_030d: [u8; 47],                          // 0x030D
    pub knife: *mut Weapon,                      // 0x033C
    pub pistol: *mut Weapon,                     // 0x0340
    pub carabine: *mut Weapon,                   // 0x0344
    pub shotgun: *mut Weapon,                    // 0x0348
    pub subgun: *mut Weapon,                     // 0x034C
    pub sniper: *mut Weapon,                     // 0x0350
    pub assault: *mut Weapon,                    // 0x0354
    pub grenade: *mut Weapon,                    // 0x0358
    pub akimbo: *mut Weapon,                     // 0x035C
    pad_0360: [u8; 4],                           // 0x0360
    pub current_weapon: *mut Weapon,             // 0x0364
    pad_0368: [u8; 12],                          // 0x0368
    pub last_attack_weapon: *mut Weapon,         // 0x0374
    pad_0378: [u8; 12],                          // 0x0378
    pub position1: Vec3,                         // 0x0384
    pub position2: Vec3,                         // 0x0390
    pub position3: Vec3,                         // 0x039C
    pub position4: Vec3,                         // 0x03A8
    pub position5: Vec3,                         // 0x03B4
    pub position6: Vec3,                         // 0x03C0
    pub position7: Vec3,                         // 0x03CC
    pad_03d8: [u8; 32],                          // 0x03D8
    pub intersectfactor1: f32,                   // 0x03F8
    pub intersectfactor2: f32,                   // 0x03FC
    pub intersectfactor3: f32,                   // 0x0400
}

#[repr(C)]
pub struct Weapon {
    pub vtable: *const *const std::ffi::c_void, // virtual table pointer
    pub id: u32,                                // 0x0004
    pub owner: *mut Player,                     // 0x0008
    pub name: *mut i8,                        // 0x000C
    pub ammo: *mut u32,                         // 0x0010
    pub mag_content: *mut u32,                  // 0x0014
    pub status: *mut WeaponStatus,              // 0x0018
    pub shooting_multiplier: u32,               // 0x001C
    pub last_reloading_frame: u32,              // 0x0020
}

#[repr(C)]
pub struct WeaponStatus {
    pub gunwait: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct CubeVec3 {
    pub vec: Vec3,
}

impl Player {
	pub fn is_dead(&self) -> bool {
		self.health <= 0 || self.player_type != 0
	}

	pub fn is_enemy(&self) -> bool {
        if GAME.is_team_game() {
			unsafe { &*GAME.local_player() }.team != self.team
		} else {
			true
		}
	}

	pub fn name(&self) -> String {
		let end = self.name.iter().position(|&c| c == 0).unwrap_or(self.name.len());
		String::from_utf8_lossy(&self.name[..end]).to_string()
	}

	pub fn is_visible(&self) -> bool {
		self.last_visible_frame == GAME.current_frame()
	}
}

impl Weapon {
    pub fn name(&self) -> String {
        unsafe {
            if self.name.is_null() {
                return String::from("Unknown");
            }
            let c_str = CStr::from_ptr(self.name);
            c_str.to_string_lossy().into_owned()
        }
    }
}