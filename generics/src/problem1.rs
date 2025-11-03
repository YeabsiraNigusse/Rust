


fn largest<T:std::cmp::PartialOrd + std::fmt::Debug> (a:T, b:T) {
    if a > b {
        println!("{:?} is larger than {:?}", a , b);
        // a
    }else{
        println!("{:?} is larger than {:?}", b , a);
        // b
    }

}
pub fn problem1(){
    // println!("hello again");
     largest(1, 2);
     largest('a', 'b');
     largest('c', 'b');

}