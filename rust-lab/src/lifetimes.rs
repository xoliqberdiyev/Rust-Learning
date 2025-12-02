pub fn lifetimes() {
    let name2 = String::from("Vali");
    let name1 = String::from("Ali");

    let result = longest(&name1, &name2);

    println!("{}", result);
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
