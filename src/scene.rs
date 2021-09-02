use super::Camera;
use super::Entity;

use glium::glutin::event::Event;
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
	pub fn event(&mut self, display: &Display, e: &Event<()>) {
		match e {
			Event::DeviceEvent {
				device_id: _,
				event,
			} => {
				self.cameras[self.active_camera_index].input(display, event);
			}
			_ => return,
		}
	}
	pub fn update(&mut self, delta_time: f32) {
		self.cameras[self.active_camera_index].update(delta_time);
		self.entities[0].transform.rotation.y += 1f32 * delta_time;
	}
}
