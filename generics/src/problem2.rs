

#[derive(Debug)]
pub struct Point<T> {
    pub x:T,
    pub y:T
}
pub fn problem2(){
    // println!("hello from problem 2");
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point{x:1.2, y:1.4};
    println!("{:?}", p1);
    println!("{:?}", p2);
}