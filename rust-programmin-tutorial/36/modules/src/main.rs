mod dcode {
    fn chicken() {
        println!("Chicken");
    }

    pub fn print_message() {
        chicken();
        println!("How's it going!");
    }

    pub mod water {
        pub fn print_message() {
            println!("I'm water!");
        }
    }
}

fn main() {
    dcode::water::print_message();
}
