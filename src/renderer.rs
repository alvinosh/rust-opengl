use super::camera::Camera;
use super::Entity;
use glium::Display;
use glium::Frame;
use glium::Program;

use glium::{glutin, uniform, Surface};

use cgmath::conv;

pub struct Renderer;

impl Renderer {
	pub fn clear(target: &mut Frame, color: (f32, f32, f32, f32), depth: f32) {
		target.clear_color_and_depth(color, depth);
	}

	pub fn render_entity(
		display: &Display,
		target: &mut Frame,
		program: &Program,
		camera: &Camera,
		entity: &Entity,
	) {
		let vb = glium::VertexBuffer::new(display, &entity.mesh.verticies).unwrap();
		let ib = glium::IndexBuffer::new(
			display,
			glium::index::PrimitiveType::TrianglesList,
			&entity.mesh.indices,
		)
		.unwrap();

		let model = entity.transform.get_model_matrix();

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
			program,
			&uniform! { model: conv::array4x4(model), view: conv::array4x4(camera.get_view()), perspective: conv::array4x4(camera.projection_matrix), u_light: light },
			&params,
		)
		.unwrap();
	}
}
