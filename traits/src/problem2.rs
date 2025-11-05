// use crate::problem1::

struct Cow<'a>{
    voice: &'a String
}
struct Cat<'a>{
    voice: &'a String
}

pub trait Speak{
    fn speak(&self) -> &String;
}

impl Speak for Cow<'_>{
    fn speak(&self) -> &String{
        self.voice
    }
}
impl Speak for Cat<'_>{
    fn speak(&self) -> &String{
        self.voice
    }
}
pub fn problem2(){
    println!("");
    let voice1 = String::from("Moo!");
    let voice2 = String::from("Meow!");

    let cow = Cat {voice: &voice1};
    let cat = Cat {voice: &voice2};

    let result1 = cow.speak();
    let result2 = cat.speak();


    println!("cow says {}", result1);
    println!("cat says {}", result2);
}