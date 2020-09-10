fn main() {
    /* Replace */
    {
        let my_string = String::from("Rust is fantastic");
        println!("After replace: {}", my_string.replace("fantastic", "great"));
    }

    /* Lines */
    {
        let my_string = String::from("The weather is\nnice\noutside mate!");

        for lines in my_string.lines() {
            println!("[ {} ]", lines);
        }
    }

    /* Split */
    {
        let my_string = String::from("Leave+a+like+if+you+enjoy!");
        let tokens: Vec<&str> = my_string.split("+").collect();

        println!("{}", my_string);
        println!("At index 2: {}", tokens[2]);
    }

    /* Trim */
    {
        let my_string = String::from("   My name is Domenic   \n\r");

        println!("Before trim: {}", my_string);
        println!("After trim: {}", my_string.trim());
    }

    /* Chars */
    {
        // let my_string = String::from("dcode on YouTube");
        let my_string = String::from("dc");
        println!("{}", my_string);

        /* Get character at index */
        match my_string.chars().nth(4) {
            Some(c) => println!("Character at index 4: {}", c),
            None => println!("No character at index 4."),
        }
    }
}
