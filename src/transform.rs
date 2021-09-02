use cgmath::{Matrix4, Vector3};

pub struct Transform {
	pub position: Vector3<f32>,
	pub rotation: Vector3<f32>,
	pub scale: Vector3<f32>,
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

	pub fn get_model_matrix(&self) -> Matrix4<f32> {
		let scale = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
		let translation = Matrix4::from_translation(self.position);
		let rotation_x = Matrix4::from_angle_x(cgmath::Rad(self.rotation.x));
		let rotation_y = Matrix4::from_angle_y(cgmath::Rad(self.rotation.y));
		let rotation_z = Matrix4::from_angle_z(cgmath::Rad(self.rotation.z));

		scale * (rotation_x * rotation_y * rotation_z) * translation
	}
}
