fn main() {
    println!("Hello, world! to the Variables in Rust");

    /* Variables */

    let x = 5; // immutable variable
    println!("The value of x is: {}", x);
    // x = 6; // error: re-assignment of immutable variable `x`
    // println!("The value of x is: {}", x);

    let mut y = 5; // mutable variable
    println!("The value of y is: {}", y);
    y = 6; // ok
    println!("The value of y is: {}", y);

    /* Constants */

    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!(
        "The value of THREE_HOURS_IN_SECONDS is: {}",
        THREE_HOURS_IN_SECONDS
    );

    /* Shadowing */

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is: {}", x);
    }
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // let mut spaces = "   "; // we’re not allowed to mutate a variable’s type
    // spaces = spaces.len();
    // println!("The value of spaces is: {}", spaces);
}
