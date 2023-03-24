use bevy::prelude::Vec3;
use rand::Rng;

pub fn generate_direction() -> Vec3 {
    let mut rnd = rand::thread_rng();

    let x_diraction = rnd.gen();
    let y_diraction = rnd.gen();

    Vec3::new(x_diraction, y_diraction, 0.0)
}
