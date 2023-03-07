fn main() {
    // addition
    let sum = 5 + 10;
        
    // subtraction
    let difference = 95.5 - 4.3;
        
    // multiplication
    let product = 4 * 30;
        
    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
        
    // remainder
    let remainder = 43 % 5;
}

// Scalar types
// Integers
/*
8-bit	i8	    u8
16-bit	i16	    u16
32-bit	i32 	u32
64-bit	i64	    u64
128-bit	i128	u128
arch	isize	usize
*/
// Floats
/*
By default floats are 64-bit, but otherwise can be annotated to 32
let x = 2.0 
let y: f32 = 3.0
*/
// Bool
/*
let t = true;
let f: bool = false;
// Char type
/*
let c = 'z';
let z: char = 'Z';
let heart_eyed_cat = '😻';
*/

// Compound types
/*
 - Tuple -
 Tuples cannot grow or shrink in size
 let tup: (i32, f64, u8) = (500, 6.4, 1);

 We can destructure a tuple like this
 fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}

We can index into a touple with .
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

- Arrays -
Arrays in Rust are fixed length, vut otherwise work like you'd expect an array to.
Arrays can be annotated like this

let a: [i32, 5] = [1, 2, 3, 4, 5];

You can initialize an array to have the same value in each element.
This makes an array of [3, 3, 3, 3, 3]

let a = [3; 5];



*/


