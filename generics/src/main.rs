mod problem1;
mod problem2;
mod problem3;

use crate::problem2::Point;


fn main() {
    // println!("Hello, world!");
    // problem1::problem1();
    // problem2::problem2();
    let p3 = Point {x: 2.3, y: 4.5};
    let res = Point::x_value(&p3);
    println!("result =  {}", res);
}
