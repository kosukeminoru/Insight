 #![windows_subsystem = "windows"]

// Using game as a separate crate

pub mod game;
pub mod db;
pub mod validation; 


fn main() {
    
 game::simulation::run();
    let attempt: game::player::Attempt = serde_json::from_str(&db::db::get(String::from("tempattempt"))).unwrap();
   
    /*
    if validation::validation::run(attempt){
        
            println!("win");
        }
        else {
           println!("lose");
     }
    */
}

/* println!("{:?}", db::db::get(String::from("tempattempt")));
let new: game::player::Attempt = serde_json::from_str(&db::db::get(String::from("tempattempt"))).unwrap();
    println!("{:?}",std::mem::size_of::<game::player::Attempt>());
    println!("{:?}", new);
    let new: game::player::MouseResource = serde_json::from_str(&db::db::get(String::from("1"))).unwrap();
    println!("{:?}", new);
    */

