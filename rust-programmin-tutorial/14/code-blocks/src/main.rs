fn main() {
    let x = 10;

    {
        // this variables only can acess in this scope (inside the brackets)
        let y = 5;

        // sucess, because this scope can acess all the variables inside and outside here
        println!("x: {} y: {}", x, y);
    }

    // error, because this scope can't acess the variable y
    println!("x: {} y: {}", x, y)
}
