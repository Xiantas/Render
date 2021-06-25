/*
	'WireFrame' consiste en un ensemble de points positionnés relativement à 'pos',
	reliés ou non par des droites, qui possède aussi une position et un rotation
*/

#![allow(non_snake_case)]

use crate::rotation::Rotation;
use crate::rotation::Coords;
use crate::camera::Camera;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

const CLIPING: f64 = -1.0/64.0;

pub struct WireFrame {
	pub pos: Coords,
	pub rot: Rotation,
	pub points: Vec<Coords>,
	pub edges: Vec<(usize, usize)>,
	pub color: Color,

	size: f64
}

impl WireFrame {
	pub fn new(pos: Coords, rot: Rotation, points: Vec<Coords>, edges: Vec<(usize, usize)>, color: Color) -> WireFrame {
		let size = points.iter().fold(-1.0, |max_val: f64, p| max_val.max(p.0*p.0 + p.1*p.1 + p.2*p.2)).sqrt();
				
		WireFrame {
			pos,
			rot,
			points,
			edges,
			color,
			size
		}
	}

	pub fn size(&self) -> f64 {
		self.size
	}

	pub fn render(&self, canvas: &mut Canvas<Window>, camera: &Camera, log: &mut String) {
		let cam_rot = camera.rot.t();
		let pos = cam_rot*(self.pos - camera.pos);
		let rot = cam_rot*self.rot;

		//Vérifie que la sphére de rayon 'size' de centre 'pos' intersecte le champ de la camera
		if pos.0.abs().max(pos.1.abs()) > self.size*(1.0+camera.fov()*camera.fov()).sqrt() - camera.fov()*pos.2 {
			return
		}

		canvas.set_draw_color(self.color);

		for edge in &self.edges {
			//Calcule la position relative des points à la caméra
			let mut pt1 = rot*self.points[edge.0] + pos;
			let mut pt2 = rot*self.points[edge.1] + pos;

			//Calcule l'intersection eventuelle du segment
			//et du plan de clipping de la caméra
			//élimant ainsi les segments qui sont derrière
			if pt1.2 <= CLIPING || pt2.2 <= CLIPING {
				if pt2.2 > CLIPING {
					pt2 = Coords(
						pt1.0 + (pt2.0-pt1.0)*(CLIPING-pt1.2)/(pt2.2-pt1.2),
						pt1.1 + (pt2.1-pt1.1)*(CLIPING-pt1.2)/(pt2.2-pt1.2),
						CLIPING);
				} else if pt1.2 > CLIPING {
					pt1 = Coords(
						pt2.0 - (pt2.0-pt1.0)*(pt2.2-CLIPING)/(pt2.2-pt1.2),
						pt2.1 - (pt2.1-pt1.1)*(pt2.2-CLIPING)/(pt2.2-pt1.2),
						CLIPING);
				}

				let mut pt1 = (
					pt1.0*500.0/((-pt1.2)*camera.fov()) + 500.,
					pt1.1*500.0/(pt1.2*camera.fov()) + 500.);
				let mut pt2 = (
					pt2.0*500.0/((-pt2.2)*camera.fov()) + 500.,
					pt2.1*500.0/(pt2.2*camera.fov()) + 500.);


				//Calcule les intersections avec le bord de l'écran
				//Pour ne rien afficher en dehors de l'écran
				if pt1.0 < 0. || 1000. < pt1.0 ||
					pt1.1 < 0. || 1000. < pt1.1 ||
					pt2.0 < 0. || 1000. < pt2.0 ||
					pt2.1 < 0. || 1000. < pt2.1 
				{
					let l = [pt1.1-pt2.1, pt2.0-pt1.0, 0., 1000.];
					let c = pt1.0*l[0] + pt1.1*l[1];
					let p = [(0, 2), (1, 2), (0, 3), (1, 3)];

					let extrems = [(pt1.0.min(pt2.0), pt1.0.max(pt2.0)), (pt1.1.min(pt2.1), pt1.1.max(pt2.1))];

					let mut points = Vec::with_capacity(2);

					for e in &p {
						if l[e.0] != 0. && extrems[1-e.0].0 <= l[e.1] && l[e.1] <= extrems[1-e.0].1 {
							let coord = (c - l[e.1]*l[1-e.0])/l[e.0];
							if 0. <= coord && coord <= 1000. {
								let paire = [coord, l[e.1]];
								points.push((paire[e.0], paire[1-e.0]));
							}
						}
					}

					match points.len() {
						1 => {
							if pt2.0 < 0. || 1000. < pt2.0 || pt2.1 < 0. || 1000. < pt2.1 {
								pt2 = points[0];
							} else {
								pt1 = points[0];
							}
						},
						2 => {
							pt1 = points[0];
							pt2 = points[1];
						},
						_ => continue
					}
				}

				let pxl1 = (pt1.0 as i32, pt1.1 as i32);
				let pxl2 = (pt2.0 as i32, pt2.1 as i32);

				log.push_str(&format!("\nDraw line [{:?}| {:?}]", pxl1, pxl2));

				canvas.draw_line(pxl1, pxl2).unwrap();
			}
		}
	}


	//Charge un ficher en vecteur de 'WireFrame'
	pub fn load_from_file<P>(file_name: P) -> Vec<WireFrame>
		where std::path::PathBuf: std::convert::From<P>
	{
		use std::fs::read;
		use std::env::{current_exe, current_dir};

		let file_name = std::path::PathBuf::from(file_name);

		let loca = {
			let mut curExeDir = current_exe().unwrap().parent().unwrap().to_path_buf();
			curExeDir.push(file_name.clone());
			if curExeDir.exists() {
				curExeDir
			} else {
				let mut curDir = current_dir().unwrap();
				curDir.push(file_name.clone());
				if curDir.exists() {
					curDir
				} else {
					panic!("File {:?} not found for the WireFrame", file_name);
				}
			}
		};

		let file = read(loca).unwrap();

		let content = String::from_utf8(file).unwrap();

		let lines: Vec<&str> = content.split('\n')
			.filter(|line| line.len() > 0 && line.chars().next().unwrap() != '#')
			.collect();

		let mut wire_frames = Vec::with_capacity(lines.len()/5);

		for i in 0..lines.len()/5 {
			let position = {
				let line: Vec<f64> = lines[5*i].split(';').take(3)
					.map(|txt| txt.trim().parse::<f64>()
						.unwrap_or_else(|_| panic!("Erreur objet #{}: « {} », une position est attendue.", i, lines[5*i]))
					).collect();
				Coords(line[0], line[1], line[2])
			};

			let rotation = {
				let line: Vec<f64> = lines[5*i+1].split(';').take(3)
					.map(|txt| txt.trim().parse::<f64>()
						.unwrap_or_else(|_| panic!("Erreur objet #{}: « {} », une rotation xyz est attendue.", i, lines[5*i+1]))
					).collect();
				Rotation::new(line[0], line[1], line[2])
			};

			let points = {
				lines[5*i+2].split('|').map(|txt| {
					let vals: Vec<f64> = txt.split(';').take(3)
						.map(|val| val.trim().parse::<f64>()
							.unwrap_or_else(|_| panic!( "Erreur objet #{}: « {} », des points sont attendus.", i, lines[5*i+2]))
						).collect();
					Coords(vals[0], vals[1], vals[2])
				}).collect()
			};

			let edges = {
				lines[5*i+3].split('|').map(|txt| {
					let vals: Vec<usize> = txt.split(';').take(2)
						.map(|val| val.trim().parse::<usize>()
							.unwrap_or_else(|_| panic!("Erreur objet #{}: « {} », des arrêtes sont attendues.", i, lines[5*i+3]))
						).collect();
					(vals[0], vals[1])
				}).collect()
			};

			let color = {
				let line: Vec<u8> = lines[5*i+4].split(';').take(4)
					.map(|txt| txt.trim().parse::<u8>()
						.unwrap_or_else(|_| panic!("Erreur objet #{}: « {} », une couleur RGBA est attendue.", i, lines[5*i+4]))
					).collect();
				Color::RGBA(line[0], line[1], line[2], line[3])
			};

			wire_frames.push(WireFrame::new(position, rotation, points, edges, color));
		}

		wire_frames
	}
}
