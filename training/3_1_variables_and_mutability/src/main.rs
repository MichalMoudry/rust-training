fn main() {
    // Variable mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    const TEST_CONSTANT: u32 = 45032;
    println!("Test constant value: {}", TEST_CONSTANT);

    // Variable shadowing
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);
}
