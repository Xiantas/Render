/*
	Définition de la caméra
	'fov' paramètre l'angle de la vision de la caméra
*/

#![allow(non_snake_case)]

use crate::rotation::{Rotation, Coords};

pub struct Camera {
	pub pos: Coords,
	pub rot: Rotation,

	fov: f64
}

impl Camera {
	pub fn new(pos: Coords, rot: Rotation, fov: f64) -> Camera {
		Camera {
			pos,
			rot,
			fov: fov.max(0.0)
		}
	}

	pub fn fov(&self) -> f64 {
		self.fov
	}

	pub fn set_fov(&mut self, new_fov: f64) {
		self.fov = new_fov.max(0.0);
	}
}
