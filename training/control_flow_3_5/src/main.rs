fn main() {
    println!("Hello, world!");
    println!("");
    first_part(7);
    println!("");
    second_part(10);
    println!("");
    println!("Loop result = {}", get_loop_result(10));
    println!("");
    while_cycle(5);
    println!("");
    for_cycle();
}

fn first_part(num: i32) {
    if num < 5 {
        println!("Number is smaller than five!");
    }
    else {
        println!("Number is larger or equal to five!");
    }
    // Inline condition
    let number = if num < 5 { 0 } else { -1 };
    println!("Test number: {}", number);
}

fn second_part(num: i32) {
    let mut count = 0;
    'counting_up: loop {
        let mut remaining = num;
        println!("count = {}", count);

        loop {
            println!("remaining = {}", remaining);
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
    println!("End count = {}", count);
}

fn get_loop_result(end: i32) -> i32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == end {
            break counter * 2;
        }
    };

    return result;
}

fn while_cycle(input: i32) {
    let mut number = input;

    while number > 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("--END--");
}

fn for_cycle() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}