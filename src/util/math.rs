#![allow(dead_code)]

pub const PI: f32 = 3.141592653589793238462;

pub struct PVector3 {
	pub x: *mut f32,
	pub y: *mut f32,
	pub z: *mut f32,
}

impl PVector3 {
	pub unsafe fn to_Vector3(&self) -> Vector3 {
		return Vector3{x: *(self.x), y: *(self.y), z: *(self.z)}
	}
}

pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vector3 {
	pub fn distance(&self, dest: Vector3) -> f32 {
		return ((dest.x - self.x)*(dest.x - self.x) + (dest.y - self.y)*(dest.y - self.y) + (dest.z - self.z)*(dest.z - self.z)).sqrt();
	}
}

pub struct Vector4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}
