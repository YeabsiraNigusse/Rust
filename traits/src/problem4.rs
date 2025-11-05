

struct Person{}

fn welcome<T: Welcome>(obj: T)-> String{
    obj.welcome()
}

struct Bot{}

trait Welcome{
    fn welcome(&self) -> String;
}

impl Welcome for Person{
    fn welcome(&self) -> String{
        String::from("welcome form person")
    }
}
impl Welcome for Bot{
    fn welcome(&self)-> String{
        String::from("welcome form Bot")
    }
}

pub fn problem4(){
    let p = Person {};
    let b = Bot {};
    let person = welcome(p);
    let bot =  welcome(b);
    println!("is the person trait comming {:?}", person);
}


// major lasson here is we can pass trait implementation function for a function
// usually we implement a trait for struct or whatever and call that function using the struct instance
// in this case ofcourse we use the struct instance but within the function and the function only accept the instance as a parameter

