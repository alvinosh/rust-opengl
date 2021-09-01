mod camera;
mod mesh;
mod shader;
mod transform;
mod window;

use camera::Camera;
use mesh::Mesh;
use shader::Shader;
use window::Window;

use cgmath::{conv, Vector3};
use glium::Display;
use glium::{glutin, uniform, Surface};

fn main() {
	let window: Window = Window::new("ViNOs");
	let display = Display::new(window.window, window.context, &window.event_loop).unwrap();
	let (width, height) = display.get_framebuffer_dimensions();

	let mesh = Mesh::from_obj_file("./res/monkey.obj");

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

	let vb = glium::VertexBuffer::new(&display, &mesh.verticies).unwrap();
	let ib = glium::IndexBuffer::new(
		&display,
		glium::index::PrimitiveType::TrianglesList,
		&mesh.indices,
	)
	.unwrap();

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
			glutin::event::Event::DeviceEvent { device_id, event } => match event {
				glutin::event::DeviceEvent::Key(input) => {
					camera.keyboard_input(&display,input);
					camera.update_pos();

				},
				glutin::event::DeviceEvent::MouseMotion { delta: (x, y) } => {
					camera.mouse_move_input(&display, (x,y));
					camera.update_pos();

				}
				_ => return,
			},
			_ => return,
		}

		let mut target = display.draw();
		target.clear_color_and_depth((0.7, 0.89, 0.92, 1.0), 1.0);

		let model = [
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 2.0, 1.0f32],
		];

		let light = [-1.0, 0.4, 0.9f32];

		let params = glium::DrawParameters {
			depth: glium::Depth {
				test: glium::draw_parameters::DepthTest::IfLess,
				write: true,
				..Default::default()
			},
			..Default::default()
		};


		target
			.draw(
				&vb,
				&ib,
				&program,
				&uniform! { model: model, view: conv::array4x4(camera.get_view()), perspective: conv::array4x4(camera.projection_matrix), u_light: light },
				&params,
			)
			.unwrap();
		target.finish().unwrap();
	});
}
