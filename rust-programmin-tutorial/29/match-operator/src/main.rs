fn main() {
    // let number = 15;
    let name = "Domenic";

    /*
    match number {
        1 => println!("It is one!"), // single match case
        // 10 | 11 => println!("It is either 10 or 11"), // 10 or 11
        2..=20 => println!("There is greater than one!"), // all betwen 2 and 20

        _ => println!("It doesn't match!"), // default action
    }
    */

    match name {
        "Chris" => println!("Nice name, mate!"),
        "Domenic" => println!("Yeah good on you, dcode!"),

        _ => println!("Don't know your name"),
    }
}
