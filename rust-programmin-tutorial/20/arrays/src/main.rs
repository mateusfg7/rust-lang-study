fn main() {
    // let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let numbers = [2; 400]; // a array of 400 number 2 -> [2, 2, 2, 2, 2...] 400x

    // numbers[0]; -> 1
    // numbers[4]; -> 5

    for i in 0..numbers.len() {
        println!("{}", numbers[i]);
    }
}
