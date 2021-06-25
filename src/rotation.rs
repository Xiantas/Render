/*
	Les rotations sont modélisées par des matrices orthogonales de dimension 3
	'Coords' représente les vecteurs de dimension 3
*/

#![allow(non_snake_case)]

#[derive(Clone, Copy)]
pub struct Rotation {
	//coefficients de la matrice rotation
	m11: f64,
	m12: f64,
	m13: f64,
	m21: f64,
	m22: f64,
	m23: f64,
	m31: f64,
	m32: f64,
	m33: f64
}

impl Rotation {
	pub fn new(ax: f64, ay: f64, az: f64) -> Rotation {
		Rotation::new_z(az)*Rotation::new_y(ay)*Rotation::new_x(ax)
	}

	pub fn new_x(ax: f64) -> Rotation {
		Rotation {
			m11: 1.0,
			m12: 0.0,
			m13: 0.0,
			m21: 0.0,
			m22: ax.cos(),
			m23: -ax.sin(),
			m31: 0.0,
			m32: ax.sin(),
			m33: ax.cos(),
		}
	}

	pub fn new_y(ay: f64) -> Rotation {
		Rotation {
			m11: ay.cos(),
			m12: 0.0,
			m13: ay.sin(),
			m21: 0.0,
			m22: 1.0,
			m23: 0.0,
			m31: -ay.sin(),
			m32: 0.0,
			m33: ay.cos(),
		}
	}

	pub fn new_z(az: f64) -> Rotation {
		Rotation {
			m11: az.cos(),
			m12: -az.sin(),
			m13: 0.0,
			m21: az.sin(),
			m22: az.cos(),
			m23: 0.0,
			m31: 0.0,
			m32: 0.0,
			m33: 1.0,
		}
	}

	pub fn t(&self) -> Rotation {
		Rotation {
			m11: self.m11,
			m12: self.m21,
			m13: self.m31,
			m21: self.m12,
			m22: self.m22,
			m23: self.m32,
			m31: self.m13,
			m32: self.m23,
			m33: self.m33
		}
	}
}

impl std::ops::Mul<Rotation> for Rotation {
	type Output = Rotation;

	fn mul(self, rhs: Rotation) -> Rotation {
		Rotation {
			m11: self.m11*rhs.m11 + self.m12*rhs.m21 + self.m13*rhs.m31,
			m12: self.m11*rhs.m12 + self.m12*rhs.m22 + self.m13*rhs.m32,
			m13: self.m11*rhs.m13 + self.m12*rhs.m23 + self.m13*rhs.m33,
			m21: self.m21*rhs.m11 + self.m22*rhs.m21 + self.m23*rhs.m31,
			m22: self.m21*rhs.m12 + self.m22*rhs.m22 + self.m23*rhs.m32,
			m23: self.m21*rhs.m13 + self.m22*rhs.m23 + self.m23*rhs.m33,
			m31: self.m31*rhs.m11 + self.m32*rhs.m21 + self.m33*rhs.m31,
			m32: self.m31*rhs.m12 + self.m32*rhs.m22 + self.m33*rhs.m32,
			m33: self.m31*rhs.m13 + self.m32*rhs.m23 + self.m33*rhs.m33
		}
	}
}

#[derive(Clone, Copy)]
pub struct Coords (pub f64, pub f64, pub f64);

impl std::ops::Add for Coords {
	type Output = Coords;

	fn add(self, rhs: Coords) -> Coords {
		Coords(
			self.0+rhs.0,
			self.1+rhs.1,
			self.2+rhs.2
		)
	}
}

impl std::ops::Sub for Coords {
	type Output = Coords;

	fn sub(self, rhs: Coords) -> Coords {
		Coords(
			self.0-rhs.0,
			self.1-rhs.1,
			self.2-rhs.2
		)
	}
}

impl std::ops::Mul<Coords> for Rotation {
	type Output = Coords;

	fn mul(self, rhs: Coords) -> Coords {
		Coords(
			self.m11*rhs.0 + self.m12*rhs.1 + self.m13*rhs.2,
			self.m21*rhs.0 + self.m22*rhs.1 + self.m23*rhs.2,
			self.m31*rhs.0 + self.m32*rhs.1 + self.m33*rhs.2
		)
	}
}
