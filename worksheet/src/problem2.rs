use std::io;



pub fn problem2(){


    // ask two numbers from the user

    println!("insert the first the number");

    let mut number1 = String::new();

    io::stdin().read_line(&mut number1).expect("the first number should be inserted");

    println!("insert the second number");

    let mut number2 = String::new();

    io::stdin().read_line(&mut number2).expect("the second number should be inserted");


    println!("your first number is {number1} and your second number is {number2}");

    // parse both of the numbers

    let number1:i32 = number1.trim().parse().expect("the first number should be parsed");

    let number2:i32 = number2.trim().parse().expect("the second number should be parsed");


    // write your logic 

    if number1 == number2 {
        println!("both numbers are equal")
    }else if number1 > number2{
        println!("number {number1} is greater than {number2}");
    }else{
        println!("number {number2} is greater than number {number1}");
    }


    // print the output


}