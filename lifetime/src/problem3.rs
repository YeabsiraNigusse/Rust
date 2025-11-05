struct Book<'a> {
    title: &'a str,
}

pub fn problem3() {
    let title = String::from("The Rust Book");
    let b = Book { title: &title };
}
