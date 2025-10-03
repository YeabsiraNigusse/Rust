use std::io;

fn main(){
    println!("guessing the number");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed to read line");

    println!("the guessd number is {guess}")


}