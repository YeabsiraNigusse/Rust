fn select_first<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len(){
        x
    }else{
        y
    }
}

pub fn problem5() {
    let s1 = String::from("alpha");
    let s2 = String::from("betayy");
    let result;
    {
        result = select_first(&s1, &s2);
    }
    println!("{}", result);
}
