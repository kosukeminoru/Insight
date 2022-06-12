 #![windows_subsystem = "windows"]

 // Using game as a separate crate

use crate::game::simulation as simulation;
pub mod game;

fn main() {
    simulation::run();

}

