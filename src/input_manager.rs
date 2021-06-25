/*
	Fichier utilisé pour réagir dynamiquement au entrées clavier
*/

#![allow(non_snake_case)]

use std::collections::HashMap;
use std::collections::hash_map::RandomState;

use sdl2::keyboard::Scancode;

#[derive(Copy, Clone)]
pub struct InputField {
	pub down: u8,
	pub up: u8,
	pub pressed: bool,
	//Pour des raisons de fonctionnement interne :
	prDown: u8,
	prUp: u8,
	liD: bool,
	lliD: bool,
}

impl InputField {
	pub fn new() -> InputField {
		InputField {
			down : 0,
			up : 0,
			pressed : false,
			prDown : 0,
			prUp : 0,
			liD : false,
			lliD : false
		}
	}
	pub fn clear(&mut self) {
		self.down = 0;
		self.up = 0;
		self.pressed = false;
		self.prUp = 0;
		self.prDown = 0;
		self.liD = false;
		self.lliD = false;
	}
}

pub struct InputSystem {
	pub fields : Vec<InputField>,
	map : HashMap<Scancode, usize, RandomState>
}

impl InputSystem {
	pub fn new(nb_fields : usize, vecBindings : &Vec<(Scancode, usize)>) -> InputSystem {
		let mut map : HashMap<Scancode, usize> = HashMap::with_capacity(vecBindings.len());
		let mut fields_checker = vec![false; nb_fields];
		for bind in vecBindings {
			if bind.1 < nb_fields {
				fields_checker[bind.1] = true;
				map.insert(bind.0, bind.1);
			} else {
				panic!("Out of range input field number : {} >= {}", bind.1, nb_fields);
			}
		}
		for i in 0..fields_checker.len() {
			if fields_checker[i] == false {
				println!("Unused {}e field, consider removing it.", i);
			}
		}
		if map.len() < vecBindings.len() {
			println!("Overlaping scancodes in vecBindings !");
		}
		map.shrink_to_fit();
		let fields = vec![InputField::new(); nb_fields];

		InputSystem {
			map,
			fields
		}
	}

	pub fn updateDownKey(&mut self, updaterKey : Scancode) {
		match self.map.get(&updaterKey) {
			Some(index) => {
				let index = *index;
				if self.fields[index].prDown != 255 {
					self.fields[index].prDown += 1;
				}
				self.fields[index].liD = true;
			},
			None => {}
		}
	}

	pub fn updateUpKey(&mut self, updaterKey : Scancode) {
		match self.map.get(&updaterKey) {
			Some(index) => {
				let index = *index;
				if self.fields[index].prUp != 255 {
					self.fields[index].prUp += 1;
				}
				self.fields[index].liD = false;
			},
			None => {}
		}
	}

	pub fn refreshFields(&mut self) {
		for inputField in &mut self.fields {
			inputField.pressed = inputField.lliD;

			if inputField.prDown != 0 {
				inputField.pressed = true;
			} else if inputField.prUp != 0 {
				inputField.pressed = false;
			}

			inputField.up = inputField.prUp;
			inputField.down = inputField.prDown;
			inputField.prUp = 0;
			inputField.prDown = 0;
			inputField.lliD = inputField.liD;
		}
	}
}
