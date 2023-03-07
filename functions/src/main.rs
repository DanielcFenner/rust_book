fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measuredment(5, 'h');
}

// function parameters must have declared types
fn another_function(x: i32) {
    println!{"The value of x is: {x}"}
}

fn print_labeled_measuredment(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// functions with a return use -> type after the parens
// functions in rust return with an expression - which does NOT have a semi-colon
fn plus_one(x: i32) -> i32 {
    x + 1
}