mod mesh;
mod transform;

use transform::Transform;
use mesh::Mesh;

struct Entity {
	transform: Transform;
	mesh: Option<Mesh>,
}
