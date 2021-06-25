#![allow(non_snake_case)]

mod rotation;
use crate::rotation::Rotation as Rot;
use crate::rotation::Coords;

mod input_manager;
use crate::input_manager::InputSystem;

mod wire_frame;
use crate::wire_frame::WireFrame;

mod camera;
use crate::camera::Camera;

use sdl2::keyboard::Scancode;
use sdl2::event::Event;
use sdl2::pixels::Color;

fn main() -> Result<(), String> {
	let sdl_context = sdl2::init()?;

	let video_subsystem = sdl_context.video()?;

	let window = video_subsystem.window("Cube", 1000, 1000)
		.position_centered()
		.opengl()
		.build()
		.map_err(|e| e.to_string())?;

	let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
	let mut event_pump = sdl_context.event_pump()?;

	let mut camera = Camera::new(
		Coords(0.0, 0.0, 0.0),
		Rot::new_x(0.0),
		0.8);

	let objects = WireFrame::load_from_file("wire_frames");


	let mut erasing = true;

	let mut inputSystem = InputSystem::new(12, &vec![
		(Scancode::W, 0),
		(Scancode::S, 1),
		(Scancode::A, 2),
		(Scancode::D, 3),
		(Scancode::Up, 4),
		(Scancode::Down, 5),
		(Scancode::Left, 6),
		(Scancode::Right, 7),
		(Scancode::Space, 8),
		(Scancode::LShift, 9),
		(Scancode::E, 10),
		(Scancode::L, 11)
	]);


	let mut lastInstant = std::time::Instant::now();
	let mut frame_duration = lastInstant.elapsed();
	let FRAME_DURATION_TARGET = std::time::Duration::from_millis(25);

	'run: loop {
		
		if lastInstant.elapsed() > frame_duration {
			frame_duration = lastInstant.elapsed() - frame_duration;
		} else {
			frame_duration = std::time::Duration::from_millis(0);
		}
		lastInstant = std::time::Instant::now();
		if frame_duration < FRAME_DURATION_TARGET {
			frame_duration = FRAME_DURATION_TARGET - frame_duration;
			std::thread::sleep(frame_duration);
		} else {
			println!("Frame duration overflow !");
		}

		//inputs
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} | Event::KeyDown { scancode: Some(Scancode::Escape), .. } => {
					break 'run
				},

				Event::KeyDown {scancode : Some(key), repeat : false, ..} => {
					inputSystem.updateDownKey(key);
				}

				Event::KeyUp {scancode : Some(key), repeat : false, ..} => {
					inputSystem.updateUpKey(key)
				}
				_ => {}
			}
		}
		inputSystem.refreshFields();

		if inputSystem.fields[0].pressed {
			camera.pos = camera.pos + camera.rot*Coords(0., 0., -0.1);
		}
		if inputSystem.fields[1].pressed {
			camera.pos = camera.pos + camera.rot*Coords(0., 0., 0.1);
		}
		if inputSystem.fields[2].pressed {
			camera.pos = camera.pos + camera.rot*Coords(-0.1, 0., 0.);
		}
		if inputSystem.fields[3].pressed {
			camera.pos = camera.pos + camera.rot*Coords(0.1, 0., 0.);
		}
		if inputSystem.fields[4].pressed {
			camera.rot = camera.rot*Rot::new_x(0.02);
		}
		if inputSystem.fields[5].pressed {
			camera.rot = camera.rot*Rot::new_x(-0.02);
		}
		if inputSystem.fields[6].pressed {
			camera.rot = camera.rot*Rot::new_y(0.02);
		}
		if inputSystem.fields[7].pressed {
			camera.rot = camera.rot*Rot::new_y(-0.02);
		}
		if inputSystem.fields[8].pressed {
			camera.pos = camera.pos + camera.rot*Coords(0., 0.1, 0.);
		}
		if inputSystem.fields[9].pressed {
			camera.pos = camera.pos + camera.rot*Coords(0., -0.1, 0.);
		}
		if inputSystem.fields[10].down != 0 {
			erasing = !erasing;
		}

		if erasing {
			canvas.set_draw_color((0, 0, 0));
			canvas.clear();
		}

		let mut log = String::from("\nLog :");

		for wire in &objects {
			wire.render(&mut canvas, &camera, &mut log);
		}

		if inputSystem.fields[11].down != 0 {
			println!("{}", log);
		}

		canvas.present();
	}

	Ok(())
}
