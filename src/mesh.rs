use glium::implement_vertex;
use obj::{load_obj, Obj};
use std::fs::File;
use std::io::BufReader;

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
	// pub fn from_obj_file<P: AsRef<std::path::Path>>(path: P) -> Mesh {
	// 	let mut output: Mesh = Mesh {
	// 		verticies: vec![],
	// 		normals: vec![],
	// 		indices: vec![],
	// 	};

	// 	for line in std::fs::read_to_string(path)
	// 		.expect("File Not Found...")
	// 		.lines()
	// 	{
	// 		let mut words = line.split_whitespace();
	// 		if let Some(word) = words.next() {
	// 			match word {
	// 				"v" => {
	// 					let verts: Vec<f32> = words.map(|x| x.parse::<f32>().unwrap()).collect();
	// 					let vertex: Vertex = Vertex {
	// 						position: (verts[0], verts[1], verts[2]),
	// 					};
	// 					output.verticies.push(vertex)
	// 				}
	// 				"f" => {
	// 					let mut indeces: Vec<u32> = words.map(|x| x.parse::<u32>().unwrap()).collect();
	// 					output.indices.append(&mut indeces);
	// 				}
	// 				_ => {}
	// 			}
	// 		};
	// 	}
	// 	return output;
	// }
}

implement_vertex!(Vertex, position, normal);
