use crate::fighter::Fighter;

pub mod fighter;

fn main() {
    let ryu = Fighter::new("Ryu", "Karate", "Japanese");

    println!("The chosen fighter is: {:?}", ryu);
}
