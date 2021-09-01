mod camera;
mod entity;
mod mesh;
mod renderer;
mod shader;
mod transform;
mod window;

use camera::Camera;
use entity::Entity;
use renderer::Renderer;
use shader::Shader;
use window::Window;

use cgmath::{conv, Vector3};
use glium::Display;
use glium::{glutin, Surface};

fn main() {
	let window: Window = Window::new("ViNOs");
	let display = Display::new(window.window, window.context, &window.event_loop).unwrap();
	let (width, height) = display.get_framebuffer_dimensions();

	let mut entity = Entity::from_obj("./res/monkey.obj");

	let camera_pos = Vector3::new(0.0, -0.0, 0.0);
	let camera_dir = Vector3::new(0.0, 0.0, 1.0);
	let camera_up = Vector3::new(0.0, 1.0, 0.0);

	let mut camera = Camera::new(
		camera_pos,
		camera_dir,
		camera_up,
		width as f32,
		height as f32,
	);

	let program =
		Shader::generate_program(&display, "./shader/basic.vert", "./shader/basic.frag", None);

	window.event_loop.run(move |event, _, control_flow| {
		match event {
			glutin::event::Event::WindowEvent { event, .. } => match event {
				glutin::event::WindowEvent::CloseRequested => {
					*control_flow = glutin::event_loop::ControlFlow::Exit;
					return;
				}
				_ => return,
			},
			glutin::event::Event::NewEvents(cause) => match cause {
				glutin::event::StartCause::ResumeTimeReached { .. } => (),
				glutin::event::StartCause::Init => (),
				_ => return,
			},
			glutin::event::Event::DeviceEvent {
				device_id: _,
				event,
			} => match event {
				glutin::event::DeviceEvent::Key(input) => {
					camera.keyboard_input(&display, input);
				}
				glutin::event::DeviceEvent::MouseMotion { delta: (x, y) } => {
					camera.mouse_move_input(&display, (x, y));
				}
				_ => return,
			},
			_ => return,
		}

		let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
		*control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

		camera.update();
		entity.transform.rotation.y += 0.01;
		entity.transform.rotation.z += 0.015;

		let mut target = display.draw();

		Renderer::clear(&mut target, (0.78, 0.88, 1.0, 1.0), 1.0);
		Renderer::render_entity(&display, &mut target, &program, &camera, &entity);

		target.finish().unwrap();
	});
}
