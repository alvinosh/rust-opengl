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

use cgmath::Vector3;
use glium::glutin::event::{Event, StartCause, WindowEvent};
use glium::Display;
use glium::{glutin, Surface};
use glutin::event_loop::ControlFlow;

fn main() {
	let window: Window = Window::new("ViNOs");
	let display = Display::new(window.window, window.context, &window.event_loop).unwrap();
	let (width, height) = display.get_framebuffer_dimensions();

	let entity = Entity::from_obj("./res/gun.obj");
	let camera = Camera::new(
		Vector3::new(0.0, 0.0, 0.0),
		Vector3::new(0.0, 0.0, 1.0),
		Vector3::new(0.0, 1.0, 0.0),
		width as f32,
		height as f32,
	);
	let mut scene = Scene::new(vec![entity], vec![camera], 0);
	let mut time = std::time::Instant::now();

	window.event_loop.run(move |event, _, control_flow| {
		*control_flow = glutin::event_loop::ControlFlow::Poll;
		scene.event(&display, &event);
		let delta_time = time.elapsed().as_secs_f32();
		scene.update(delta_time);
		time = std::time::Instant::now();

		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => {
					*control_flow = ControlFlow::Exit;
					return;
				}
				_ => (),
			},
			Event::NewEvents(cause) => match cause {
				StartCause::ResumeTimeReached { .. } => (),
				StartCause::Init => (),
				_ => (),
			},
			Event::RedrawRequested(_) => {
				let mut target = display.draw();
				Renderer::clear(&mut target, (0.78, 0.88, 1.0, 1.0), 1.0);
				Renderer::render_scene(&display, &mut target, &scene);
				target.finish().unwrap();
			}
			_ => (),
		};
	});
}
