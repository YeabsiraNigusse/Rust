// the following method solve by passing the varibale as a reference so it cant be moved/loose it ownership

// fn take_ownership(s: &String) {
//     println!("Taking ownership of {}", s);
// }

// pub fn problem3() {
//     let s = String::from("Rustacean");
//     take_ownership(&s);
//     println!("{}", s);
// }



// this is just the defualt.


fn take_ownership(s: String) {
    println!("Taking ownership of {}", s);
}

pub fn problem3() {
    let s = String::from("Rustacean");
    let s = take_ownership(s);
    println!("{:?}", s);
}
