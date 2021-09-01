pub struct Shader {}

impl Shader {
	pub fn generate_program<P: AsRef<std::path::Path>>(
		display: &glium::Display,
		vert_shader: P,
		frag_shader: P,
		_geometry_shader: Option<P>,
	) -> glium::Program {
		let vertex_shader_src = std::fs::read_to_string(vert_shader).unwrap();
		let fragment_shader_src = std::fs::read_to_string(frag_shader).unwrap();

		glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap()
	}
}
