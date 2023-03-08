fn main() {
    //if_statements();
    //loops();
    loop_break();
    loop_label();
    while_loops();
    for_loops();
}

fn for_loops() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // we can also iterate over a range
    // this is in reverse because rev()
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("lift off");
}

fn while_loops() {
    let mut number: i32 = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// We can label loops so we can break out of a loop
// within a loop
fn loop_label() {
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn loop_break() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{result}");
}

fn loops() {
    loop {
        println!("again!");
    }
}

fn if_statements() {
    let number: i32 = 7;

    if number % 4 == 0 {
        println!("number is divisible by 4")
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // we can assign variables with if statements since they're just expressions
    // the return types from each if arm must be the same type or we'll get an error
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("Value of number is {number}")
}
