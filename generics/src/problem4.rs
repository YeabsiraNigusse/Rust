use std::ops::Mul;


fn print_double<T: std::ops::Mul<i32>>(value: T)  where <T as Mul<i32>>::Output: std::fmt::Display {
    println!("Double: {}", value * 2);
}

pub fn problem4() {
    print_double(5);
}
