use std::io;

pub fn problem3(){

    println!("insert 3 numbers in a row");

    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut num3 = String::new();

    io::stdin().read_line(&mut num1).expect("number1 should be inserted");
    io::stdin().read_line(&mut num2).expect("number2 should be inserted");
    io::stdin().read_line(&mut num3).expect("number3 should be inserted");


    let num1:i32 = num1.trim().parse().expect("number1 should be parsed");
    let num2:i32 = num2.trim().parse().expect("number2 should be parsed");
    let num3:i32 = num3.trim().parse().expect("number3 should be parsed");


    let mut v = [num1, num2, num3];

    v.sort();
    // assert_eq!(v, [1, 2, 3]);

    println!("ordered number {:?}", v);

}