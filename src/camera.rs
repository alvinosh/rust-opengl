use cgmath::prelude::*;
use cgmath::{Deg, Matrix4, Vector3, Vector4};
use glium::glutin::event::DeviceEvent;
use glium::glutin::event::ElementState;
use glium::glutin::event::KeyboardInput;
use glium::glutin::event::VirtualKeyCode;

use glium::Display;

pub struct Camera {
	pub projection_matrix: Matrix4<f32>,
	pub position: Vector3<f32>,
	pub orientation: Vector3<f32>,
	pub up: Vector3<f32>,
	pub speed: f32,
	pub sens: f32,
	pub yaw: f32,
	pitch: f32,
	lock: bool,

	forward: f32,
	side: f32,
	top: f32,
}

impl Camera {
	pub fn new(
		position: Vector3<f32>,
		orientation: Vector3<f32>,
		up: Vector3<f32>,
		width: f32,
		height: f32,
	) -> Camera {
		let pm = {
			let aspect_ratio = height / width;

			let fov: f32 = 3.141592 / 3.0;
			let zfar = 1024.0;
			let znear = 0.1;

			let f = 1.0 / (fov / 2.0).tan();

			Matrix4::from_cols(
				Vector4::new(f * aspect_ratio, 0.0, 0.0, 0.0),
				Vector4::new(0.0, f, 0.0, 0.0),
				Vector4::new(0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0),
				Vector4::new(0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0),
			)
		};

		Camera {
			projection_matrix: pm,
			position: position,
			orientation: orientation,
			up: up,
			speed: 10f32,
			sens: 0.1f32,
			yaw: 0f32,
			pitch: 0f32,
			lock: false,

			forward: 0.0,
			side: 0.0,
			top: 0.0,
		}
	}

	pub fn get_view(&self) -> Matrix4<f32> {
		Camera::view_matrix(&self.position, &self.orientation, &self.up)
	}

	pub fn update(&mut self, delta_time: f32) {
		self.position += self.speed * delta_time * self.forward * self.orientation.normalize();
		self.position +=
			self.speed * delta_time * self.side * self.orientation.cross(self.up).normalize();
		self.position += self.speed * delta_time * self.top * self.up;

		self.orientation.x = Deg::cos(Deg(self.yaw)) * Deg::cos(Deg(self.pitch));
		self.orientation.y = Deg::sin(Deg(self.pitch));
		self.orientation.z = Deg::sin(Deg(self.yaw)) * Deg::cos(Deg(self.pitch));

		self.orientation = self.orientation.normalize();
	}

	pub fn input(&mut self, display: &Display, key: &DeviceEvent) {
		match key {
			DeviceEvent::Key(input) => {
				let keycode = input.virtual_keycode.unwrap();
				match keycode {
					VirtualKeyCode::W => match input.state {
						ElementState::Pressed => self.forward = 1.0,
						ElementState::Released => self.forward = 0.0,
					},
					VirtualKeyCode::S => match input.state {
						ElementState::Pressed => self.forward = -1.0,
						ElementState::Released => self.forward = 0.0,
					},
					VirtualKeyCode::A => match input.state {
						ElementState::Pressed => self.side = 1.0,
						ElementState::Released => self.side = 0.0,
					},
					VirtualKeyCode::D => match input.state {
						ElementState::Pressed => self.side = -1.0,
						ElementState::Released => self.side = 0.0,
					},
					VirtualKeyCode::Space => match input.state {
						ElementState::Pressed => self.top = 1.0,
						ElementState::Released => self.top = 0.0,
					},
					VirtualKeyCode::LControl => match input.state {
						ElementState::Pressed => self.top = -1.0,
						ElementState::Released => self.top = 0.0,
					},
					VirtualKeyCode::Escape => {
						if input.state == ElementState::Released {
							self.lock = !self.lock
						}
					}
					_ => return,
				}
			}
			DeviceEvent::MouseMotion { delta: (x, y) } => {
				let win = display.gl_window();
				let size = win.window().inner_position().unwrap();
				win.window().set_cursor_visible(false);
				if self.lock {
					win.window().set_cursor_position(size).unwrap();
				}
				self.yaw -= *x as f32 * self.sens;
				self.pitch -= *y as f32 * self.sens;
				if self.pitch > 89.0 {
					self.pitch = 89.0;
				}
				if self.pitch < -89.0 {
					self.pitch = -89.0;
				}
			}
			_ => return,
		}
	}

	fn view_matrix(
		position: &Vector3<f32>,
		direction: &Vector3<f32>,
		up: &Vector3<f32>,
	) -> Matrix4<f32> {
		let f = {
			let f = direction;
			let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
			let len = len.sqrt();
			[f[0] / len, f[1] / len, f[2] / len]
		};
		let s = [
			up[1] * f[2] - up[2] * f[1],
			up[2] * f[0] - up[0] * f[2],
			up[0] * f[1] - up[1] * f[0],
		];
		let s_norm = {
			let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
			let len = len.sqrt();
			[s[0] / len, s[1] / len, s[2] / len]
		};
		let u = [
			f[1] * s_norm[2] - f[2] * s_norm[1],
			f[2] * s_norm[0] - f[0] * s_norm[2],
			f[0] * s_norm[1] - f[1] * s_norm[0],
		];
		let p = [
			-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
			-position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
			-position[0] * f[0] - position[1] * f[1] - position[2] * f[2],
		];

		Matrix4::from_cols(
			Vector4::new(s_norm[0], u[0], f[0], 0.0),
			Vector4::new(s_norm[1], u[1], f[1], 0.0),
			Vector4::new(s_norm[2], u[2], f[2], 0.0),
			Vector4::new(p[0], p[1], p[2], 1.0),
		)
	}
}
