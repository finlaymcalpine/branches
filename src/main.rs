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
    let truth = if number % 2 == 0 {"is"} else {"is not"};
    println!("number {truth} divisible by 2");
}