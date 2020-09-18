struct Rectangle {
    width: u8,
    height: u8,
}

impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    println!("Hello, world!");
}

fn give_two() -> i32 {
    2
}

#[cfg(test)]
mod dcode_tests {
    // THE TEST WILL BE SUCESS IF THE FUNCTION IS TRUE
    #[test]
    fn test_basic() {
        assert!(1 == 1);
    }

    // THE TEST WILL BE TRUE IF THE FUNCTION IS PANIC
    #[test]
    #[should_panic]
    fn test_panic() {
        assert!(1 == 1);
        panic!("Oh no!");
    }

    #[test]
    fn test_equals() {
        assert_eq!(2, 1 + 1); // assert equals
        assert_ne!(2, 1 + 2); // assert not equals
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(2, 1 + 1); // assert equals
        assert_ne!(2, 1 + 2); // assert not equals
    }

    #[test]
    fn extern_fn_test() {
        assert_eq!(super::give_two(), 1 + 1); // assert equals
        assert_ne!(super::give_two(), 1 + 2); // assert not equals
    }

    #[test]
    #[should_panic]
    fn test_structs() {
        let r = super::Rectangle {
            width: 50,
            height: 25,
        };

        assert!(r.is_square());
    }
}
