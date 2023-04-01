use phf::phf_map;
use rand::{self, Rng};

static FOOD_INDEX_TO_IMAGE_PATH_MAP: phf::Map<u8, &str> = phf_map! {
    1u8 => "images/food/banana.png",
    2u8 => "images/food/salad.png",
    3u8 => "images/food/grass.png",
    4u8 => "images/food/watermelon.png",
    5u8 => "images/food/strawberry.png",
};

pub fn get_food_sprite_path<'a>() -> &'a str {
    let mut rnd = rand::thread_rng();

    let food_sprite_index = rnd.gen_range(1..=5) as u8;
    let path = FOOD_INDEX_TO_IMAGE_PATH_MAP
        .get(&food_sprite_index)
        .unwrap();
    path
}
