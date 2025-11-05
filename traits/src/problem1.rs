
pub trait Speak{
    fn speak(&self) -> &String;
}

struct Dog <'a>{
    name: &'a String,
    voice: &'a String
}

impl Speak for Dog<'_>{ // why do we need liftime here?
    fn speak(&self)-> &String{
        self.voice
    }
}

pub fn problem1(){

     let name = String::from("Bob");
     let voice = String::from("Woof!");

     let dog = Dog {name: &name, voice: &voice};
     let result =  dog.speak();
     println!("{}", result);
}