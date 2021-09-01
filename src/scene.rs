use super::Camera;
use super::Entity;

use glium::glutin::event::DeviceEvent;
use glium::Display;

pub struct Scene {
	pub entities: Vec<Entity>,
	pub cameras: Vec<Camera>,
	pub active_camera_index: usize,
}

impl Scene {
	pub fn new(entities: Vec<Entity>, cameras: Vec<Camera>, active: usize) -> Scene {
		Scene {
			entities: entities,
			cameras: cameras,
			active_camera_index: active,
		}
	}
	pub fn event(&mut self, display: &Display, event: DeviceEvent) {
		match event {
			DeviceEvent::Key(input) => {
				self.cameras[self.active_camera_index].keyboard_input(display, input);
			}
			DeviceEvent::MouseMotion { delta: (x, y) } => {
				self.cameras[self.active_camera_index].mouse_move_input(display, (x, y));
			}
			_ => return,
		}
	}
	pub fn update(&mut self) {
		self.cameras[self.active_camera_index].update();
		self.entities[0].transform.rotation.y += 0.01;
	}
}
