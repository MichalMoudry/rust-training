fn main() {
    println!("Hello, world!");
    first_part(5);
}

fn first_part(num: i32) {
    if num < 5 {
        println!("Number is smaller than five!");
    }
    else {
        println!("Number is larger or equal to five!");
    }
}