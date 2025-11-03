

fn swap<T:Copy>(a: &mut T, b: &mut T){

    let temp = *a;
    *a = *b;
    *b = temp;

}

pub fn problem6(){
    let mut x = 10;
    let mut y = 20; 

    println!("before x = {}, before y = {}", x , y);
    swap(&mut x, &mut y);
    println!("after x = {}, after y = {}", x , y);

}

