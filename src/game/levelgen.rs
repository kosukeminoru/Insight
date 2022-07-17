use crate::game::constants::WX;
use crate::game::constants::WY;
use rand::prelude::*;
//use rand_seeder::{Seeder, SipHasher};
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

//not done yet
pub fn generate(hash: String, difficulty: u8) {
    //-> Level {
    let dif = difficulty as f32;
    let mut rng: Pcg64 = Seeder::from(hash).make_rng();
    let height: f32 = (WX * WY / dif).sqrt().round() - 1.0;
    let width: f32 = (WX * WY / dif).sqrt().round() - 1.0;
    //let width: f32 =

    println!("{}", height);
    println!("{}", width);
    println!("{}", rng.gen_range(0..100));
}
