use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("guess the number");

    let number = rand::thread_rng().gen_range(1..=100);

    println!("the secret number is {number}");

   
    loop{
        println!("Please input your guess.");
        
        
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("failed to read line");
        
        println!("the guessd number is {guess}");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } 
                
          }

    }
    


}