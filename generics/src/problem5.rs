

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn problem5(){
    let result;
    let x = String::from("hello");
    // {
     let y = String::from("world");
     result = longest(&x, &y);
    // }
    println!("{}", result);
    
}


// what the golader rule it should be here(i belive)

// if we always have more than one refence parameter and if we are returning more than one based on the condition or whatever
    // we should tie the return value with the function parameter using lifetime spcifier so
    // whenever we use that functionwe make sure we are not gonna use out of scope parameter and  returned value