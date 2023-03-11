fn main() {
    let mut name: String = String::new();
    name = "dan".to_string();
    print(&name);

    let mut string = String::from("foo");
    append_to_string(&mut string);
    print(&string);
    print(&string);
}
fn append_to_string(string: &mut String) {
    string.push_str("something");
}
fn print(arg: &String) {
    println!("{arg}");
}
