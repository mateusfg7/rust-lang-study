use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    // add values
    marks.insert("Rust Programming", 96);
    marks.insert("Web Development", 94);
    marks.insert("UX Design", 75);
    marks.insert("Professional Computer Studies", 45);

    // find length of HashMap
    println!("How many subjects have you studies? {}", marks.len());

    // get single value
    match marks.get("Web Development") {
        Some(mark) => println!("You got {} for Web Dev!", mark),
        None => println!("You didn't study Web Development"),
    }

    // remove a value
    marks.remove("UX Design");

    // loop through HashMap
    for (subject, mark) in &marks {
        println!("For {} you got {}%!", subject, mark)
    }

    // check for value
    println!(
        "Did you study C++? {}",
        marks.contains_key("C++ Programming")
    );
}
