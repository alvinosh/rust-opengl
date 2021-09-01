use cgmath::prelude::*;
use cgmath::{Matrix4, Rad, Vector3, Vector4};
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
	yaw: f32,
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
			speed: 0.1,
			sens: 0.01f32,
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

	pub fn update(&mut self) {
		self.position += self.orientation * self.speed * self.forward;
		self.position += self.speed * self.orientation.cross(self.up).normalize() * self.side;
		self.position += self.up * self.speed * self.top;

		self.orientation.x = Rad::cos(Rad(self.yaw)) * Rad::cos(Rad(self.pitch));
		self.orientation.y = Rad::sin(Rad(self.pitch));
		self.orientation.z = Rad::sin(Rad(self.yaw)) * Rad::cos(Rad(self.pitch));
		self.orientation = self.orientation.normalize();
	}

	pub fn keyboard_input(&mut self, _display: &Display, key: KeyboardInput) {
		let keycode = key.virtual_keycode.unwrap();
		match keycode {
			VirtualKeyCode::W => match key.state {
				ElementState::Pressed => self.forward = 1.0,
				ElementState::Released => self.forward = 0.0,
			},
			VirtualKeyCode::S => match key.state {
				ElementState::Pressed => self.forward = -1.0,
				ElementState::Released => self.forward = 0.0,
			},
			VirtualKeyCode::A => match key.state {
				ElementState::Pressed => self.side = 1.0,
				ElementState::Released => self.side = 0.0,
			},
			VirtualKeyCode::D => match key.state {
				ElementState::Pressed => self.side = -1.0,
				ElementState::Released => self.side = 0.0,
			},
			VirtualKeyCode::Space => match key.state {
				ElementState::Pressed => self.top = 1.0,
				ElementState::Released => self.top = 0.0,
			},
			VirtualKeyCode::LControl => match key.state {
				ElementState::Pressed => self.top = -1.0,
				ElementState::Released => self.top = 0.0,
			},
			VirtualKeyCode::Escape => {
				if key.state == ElementState::Released {
					self.lock = !self.lock
				}
			}

			_ => return,
		}
	}
	pub fn mouse_move_input(&mut self, display: &Display, delta: (f64, f64)) {
		println!("pitch {} , yaw {}", self.pitch, self.yaw);
		let win = display.gl_window();
		let size = win.window().inner_position().unwrap();
		win.window().set_cursor_visible(false);
		if self.lock {
			win.window().set_cursor_position(size).unwrap();
		}

		self.yaw -= delta.0 as f32 * self.sens;
		self.pitch += delta.1 as f32 * self.sens;
		if self.pitch > 89.0 {
			self.pitch = 89.0;
		}
		if self.pitch < -89.0 {
			self.pitch = -89.0;
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
