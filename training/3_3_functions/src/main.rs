fn main() {
    println!("Hello, world!");

    another_function(50, 'h');
    print!("Addition 5 + 6 = {}", addition(5, 6));
}

fn another_function(x: i32, unit_label: char) {
    println!("Another function. Value of x: {}", x);
    println!("{}{}", x, unit_label);
    let y = x + 1;
    println!("y = {}", y);
}

fn addition(x: i32, y: i32) -> i32 {
    return x + y;
}