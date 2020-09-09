fn main() {
    // diferent types in one tupĺe
    let tup1 = (20, "Rust", 2.4, false, (1, 4, 7));

    println!("{}", (tup1.4).1);

    // desustructuring
    let tup2 = (45, 6.7, "Computer");
    let (a, b, c) = tup2;

    println!("a is {}, b is {} and c is {}", a, b, c)
}
