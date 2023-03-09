fn main() {
    println!("{}", nth_fibonacci_vec(10));
    println!("{}", nth_fibonacci(10));
}

fn nth_fibonacci_vec(n: usize) -> u128 {
    let mut fib_vec: Vec<u128> = [1, 1].to_vec();

    for ele in 2..n {
        fib_vec.push(fib_vec[ele - 1] + fib_vec[ele - 2]);
    }

    fib_vec[fib_vec.len() - 1]
}

fn nth_fibonacci(n: i32) -> u128 {
    let mut first_number: u128 = 0;
    let mut second_number: u128 = 0;
    let mut current_number: u128 = 1;

    let mut i = 1;

    while i < n {
        first_number = second_number;

        second_number = current_number;

        current_number = first_number + second_number;

        i += 1;

        println!("{current_number}");
    }

    current_number
}
