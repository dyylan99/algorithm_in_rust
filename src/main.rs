use crate::bit::is_unique::is_unique;

pub mod bit;
mod graph;
mod hashtable;
mod string;


fn main() {
    let is_unique = is_unique("abcc".to_string());
    println!("{}", is_unique);
    println!("Hello, world!");
}
