fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


pub fn problem2(){
    let str1 = String::from("hello");
    let str2 = String::from("world");

    longest(&str1, &str2);
}