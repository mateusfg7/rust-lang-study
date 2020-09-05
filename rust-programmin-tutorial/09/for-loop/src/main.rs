fn main() {
    let _numbers = 30..51; // range of numbers, from 30 to 51

    let animals = vec!["Rabbit", "Dog", "Cat"];

    for (index, a) in animals.iter().enumerate() {
        println!("The index is {} and the animal is {}", index, a)
    }
}
