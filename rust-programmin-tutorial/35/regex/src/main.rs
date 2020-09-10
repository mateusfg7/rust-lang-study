extern crate regex;
use regex::Regex;

fn main() {
    /*{
        let re = Regex::new(r"\w{5}").unwrap();
        let text = "dcode";

        println!("Found match? {}", re.is_match(text))
    }*/

    {
        let re = Regex::new(r"(\w{5})").unwrap();
        let text = "dcode";

        match re.captures(text) {
            // Some(caps) => println!("Found Match: {}", caps.get(0).unwrap().as_str()),
            Some(caps) => println!("Found Match: {}", &caps[0]),
            None => println!("Could not find match..."),
        }
    }
}
