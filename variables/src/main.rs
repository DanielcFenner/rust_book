// Constants are written in caps with underscores
// Constants MUST have the type annotated
// Constants cannot change *ever*
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Variables by default cannot be changed, unless mut is added
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
