fn get_ref() -> String {
    let s = String::from("data");
    s
}

pub fn problem4() {
    let r = get_ref();
    println!("{}", r);
}
