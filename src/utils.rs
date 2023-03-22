use bevy::prelude::Vec3;
use phf::phf_map;
use rand::{self, Rng};

static FOOD_INDEX_TO_IMAGE_PATH_MAP: phf::Map<u8, &str> = phf_map! {
    1u8 => "food/banana.png",
    2u8 => "food/salad.png",
    3u8 => "food/salad2.png",
    4u8 => "food/watermelon.png",
    5u8 => "food/strawberry.png",
};

pub fn get_food_sprite_path<'a>() -> &'a str {
    let mut rnd = rand::thread_rng();

    let food_sprite_index = rnd.gen_range(1..=5) as u8;
    let path = FOOD_INDEX_TO_IMAGE_PATH_MAP
        .get(&food_sprite_index)
        .unwrap();
    path
}

pub fn generate_direction() -> Vec3 {
    let mut rnd = rand::thread_rng();

    let x_diraction = rnd.gen();
    let y_diraction = rnd.gen();

    Vec3::new(x_diraction, y_diraction, 0.0)
}
