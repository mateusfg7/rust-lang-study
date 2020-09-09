fn main() {
    let mut my_string = String::from("How's it going? My name is Dom.");

    // Lenght
    println!("Lenght: {}", my_string.len());

    // Is empty?
    println!("String is empty? {}", my_string.is_empty());

    for token in my_string.split_whitespace() {
        println!("{}", token);
    }

    println!(
        "Does the string contain 'Dom'? {}",
        my_string.contains("Dom")
    );

    my_string.push_str(" Welcome to your tutorial on Strings!");

    println!("{}", my_string);
}
