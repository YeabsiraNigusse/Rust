
struct Robot;

pub trait Greet{
    fn greet(&self){
        println!("Hello World");
    }
}

// impl Greet for Robot{} // default trait implementation
impl Greet for Robot{
    fn greet(&self){
        println!("greeting Human!!");
    }
}

pub fn problem3(){
   let robot = Robot {};
   robot.greet();
   
}

// we can impelement default trait call or overide it