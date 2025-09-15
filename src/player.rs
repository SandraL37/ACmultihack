#[repr(C)]
#[derive(Debug)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct Player {
	pub pad_0000: [u8; 4], //0x0000
	pub head: Vector3, //0x0004 READONLY
	pub sprint: Vector3, //0x0010
	pub mult_x: u32, //0x001C READONLY
	pub mult_y: u32, //0x0020 READONLY
	pub mult_z: u32, //0x0024 READONLY
	pub velocity: Vector3, //0x0028 READONLY
	pub position: Vector3, //0x0034
	pub yaw: f32, //0x0040
	pub pitch: f32, //0x0044
	pub pad_0048: [u8; 176], //0x0048
	pub health: i32, //0x00F8
	pub pad_00fc: [u8; 628], //0x00FC
	pub current_weapon: *mut Weapon, //0x0370
}

#[repr(C)]
#[derive(Debug)]
pub struct Weapon {
	pub pad_0000: [u8; 12], //0x0000
	pub info: *mut WeaponInfo, //0x000C
	pub current_mag: *mut i32, //0x0010
	pub current_ammo: *mut u32, //0x0014
	pub current_time: *mut u32, //0x0018
	pub pad_001c: [u8; 4], //0x001C
}

#[repr(C)]
#[derive(Debug)]
pub struct WeaponInfo {
	pub pad_0000: [u8; 260], //0x0000
	pub n00000421: u16, //0x0104
	pub n00000467: u16, //0x0106
	pub reloadtime: u16, //0x0108
	pub firerate: u16, //0x010A
	pub damage: u16, //0x010C
	pub n0000046a: u16, //0x010E
	pub pad_0110: [u8; 4], //0x0110
	pub spread: u16, //0x0114
	pub kickback: u16, //0x0116
	pub mag_size: u16, //0x0118
	pub weapon_pushup: u16, //0x011A
	pub weapon_pushback: u16, //0x011C
	pub unknown: u16, //0x011E
	pub recoil_1: u16, //0x0120
	pub recoil_2: u16, //0x0122
	pub pushdown_after_fire: u16, //0x0124
	pub automatic_1: u16, //0x0126
	pub automatic_2: u16, //0x0128
}