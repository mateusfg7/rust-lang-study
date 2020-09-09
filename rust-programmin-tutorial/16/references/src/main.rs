fn main() {
    let mut x = 10;

    let _xr = &x;

    {
        let dom = &mut x;
        *dom += 1;
    }

    println!("x is {}", x)
}
