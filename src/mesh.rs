use glium::implement_vertex;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
	pub position: (f32, f32, f32),
	pub normal: (f32, f32, f32),
}

#[derive(Clone, Debug)]
pub struct Mesh {
	pub verticies: Vec<Vertex>,
	pub indices: Vec<u32>,
}

impl Mesh {
	pub fn from_obj_file<P: AsRef<std::path::Path>>(path: P) -> Mesh {
		let mut output: Mesh = Mesh {
			verticies: vec![],
			indices: vec![],
		};

		let mut vertex_pos: Vec<(f32, f32, f32)> = vec![];
		let mut normals: Vec<(f32, f32, f32)> = vec![];

		let mut current_index = 0;

		for line in std::fs::read_to_string(path)
			.expect("File Not Found...")
			.lines()
		{
			let mut words = line.split_whitespace();
			if let Some(word) = words.next() {
				match word {
					"v" => {
						let verts: Vec<f32> = words.map(|x| x.parse::<f32>().unwrap()).collect();
						vertex_pos.push((verts[0], verts[1], verts[2]));
					}
					"vn" => {
						let nrms: Vec<f32> = words.map(|x| x.parse::<f32>().unwrap()).collect();
						normals.push((nrms[0], nrms[1], nrms[2]));
					}
					"f" => {
						for face in words {
							let indexes: Vec<f32> = face.split("/").map(|x| x.parse::<f32>().unwrap()).collect();
							let v = Vertex {
								position: vertex_pos[indexes[0] as usize - 1],
								normal: normals[indexes[2] as usize - 1],
							};
							output.verticies.push(v);
							output.indices.push(current_index);
							current_index += 1;
						}
					}
					_ => {}
				}
			};
		}
		return output;
	}
}

implement_vertex!(Vertex, position, normal);
