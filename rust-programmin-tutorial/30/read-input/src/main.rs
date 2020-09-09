use std::io;

fn main() {
    let mut input = String::new();

    println!("Hey mate! say something:");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Sucess! You said: {}", input.to_uppercase());
        }
        Err(e) => println!("Oops! something went wrong: {}", e),
    }
}
