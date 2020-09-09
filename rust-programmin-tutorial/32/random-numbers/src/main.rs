extern crate rand;
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1, 11); //1-11
    println!("Random Number: {}", random_number);

    let random_bool = rand::thread_rng().gen_weighted_bool(2);
    println!("Random Boolean: {}", random_bool);
}
