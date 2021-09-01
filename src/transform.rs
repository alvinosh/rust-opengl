use cgmath::Vector3;

pub struct Transform {
	position: Vector3<f32>,
	rotation: Vector3<f32>,
	scale: Vector3<f32>,
}

impl Transform {
	pub fn new(
		pos: Option<Vector3<f32>>,
		rot: Option<Vector3<f32>>,
		scale: Option<Vector3<f32>>,
	) -> Transform {
		Transform {
			position: {
				match pos {
					Some(p) => p,
					None => Vector3::new(0.0, 0.0, 0.0),
				}
			},
			rotation: {
				match rot {
					Some(p) => p,
					None => Vector3::new(0.0, 0.0, 0.0),
				}
			},
			scale: {
				match scale {
					Some(p) => p,
					None => Vector3::new(1.0, 1.0, 1.0),
				}
			},
		}
	}
}
