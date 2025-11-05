

struct Circle{
    radius:f64
}

struct Square{
    side:f64
}

trait Area {
    fn area(&self)->f64;
}

impl Area for Circle{
    fn area(&self) -> f64{
        self.radius * self.radius * 3.14
    }
}
impl Area for Square{
    fn area(&self) -> f64{
        self.side * self.side 
    }
}

fn print_area<T: Area>(shape: T){
    let area = shape.area();
    println!("are is {}", area);
}

pub fn problem5(){
    let a = Circle { radius: 3.0 };
    let b = Square { side: 4.0 };

    print_area(a);
    print_area(b);
}