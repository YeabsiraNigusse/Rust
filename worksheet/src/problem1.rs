use std::io;

pub fn problem1() {
    // println!("Hello problem one");

    println!("please insert a number");

    // define a variable that store the commandline input

    let mut number = String::new();

    io::stdin().read_line(&mut number).expect("user should insert a number");

    // convert it to integer number

    let number:u32 = number.trim().parse().expect("it should be parsed");


    // write your logic here

    if number % 2 == 0 {
        println!("the number {number} is even");
    }
    else{
        println!("the number {number} is odd");
    }

    // print your statment that show the number is weather it is even or odd.
}


