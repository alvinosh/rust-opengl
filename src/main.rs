mod camera;
mod entity;
mod mesh;
mod renderer;
mod scene;
mod shader;
mod transform;
mod window;

use camera::Camera;
use entity::Entity;
use renderer::Renderer;
use scene::Scene;
use shader::Shader;
use window::Window;

use cgmath::{conv, Vector3};
use glium::Display;
use glium::{glutin, Surface};

fn main() {
	let window: Window = Window::new("ViNOs");
	let display = Display::new(window.window, window.context, &window.event_loop).unwrap();
	let (width, height) = display.get_framebuffer_dimensions();

	let entity = Entity::from_obj("./res/monkey.obj");
	let camera = Camera::new(
		Vector3::new(0.0, 0.0, 0.0),
		Vector3::new(0.0, 0.0, 1.0),
		Vector3::new(0.0, 1.0, 0.0),
		width as f32,
		height as f32,
	);

	let mut scene = Scene::new(vec![entity], vec![camera], 0);

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
			} => scene.event(&display, event),
			_ => return,
		};

		let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
		*control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

		scene.update();

		let mut target = display.draw();
		Renderer::clear(&mut target, (0.78, 0.88, 1.0, 1.0), 1.0);
		Renderer::render_scene(&display, &mut target, &scene);
		target.finish().unwrap();
	});
}
