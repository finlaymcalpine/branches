fn main() {
    let number = 9;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2")
    } else {
        println!("number is not divisible by 4, 3, or 2")
    }

    // since if is an expression, we can use it in a let statement to assign a variable based on outcome
    // each arm of the if must be of the same type, otherwise we get a compiler error
    // this is because the compiler wants to know what type truth will be, but can't if the arms differ
    let truth = if number % 2 == 0 { "is" } else { "is not" };
    println!("number {truth} divisible by 2");

    looper();

    multi_looper();

    whiler();

    for_loop();

    range_loop();
}

fn looper() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}")
}

fn multi_looper() {
    let mut count = 0;
    
    // counting_up is a loop label, marking the name of a loop for breaking or continuing. It needs to start with a single quote
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

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
    
    // count remains in scope after loop termination, so we can call on count in the string below.
    println!("End count = {count}");
}

fn whiler() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// we can loop through an array using a while loop and an index counter, but that involves bounds checking each time.
// instead, can use a for loop on the array like below
fn for_loop() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("the value is: {element}");
    }
}

// can lso use the Range provided in std library to create a range for iterating
fn range_loop() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}