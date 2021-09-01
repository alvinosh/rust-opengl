use super::mesh::Mesh;
use super::transform::Transform;

pub struct Entity {
	pub transform: Transform,
	pub mesh: Mesh,
}

impl Entity {
	pub fn from_obj<P: AsRef<std::path::Path>>(path: P) -> Entity {
		Entity {
			transform: Transform::new(None, None, None),
			mesh: Mesh::from_obj_file(path),
		}
	}
}
