fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true!");
    } else {
        println!("Condition was false!");
    }


    // multiple conditions

    let x = 6;

    if x % 4 == 0 {
        println!("x is divisible by 4");
    } else if x % 3 == 0 {
        println!("x is divisible by 3");
    } else if x % 2 == 0 {
        println!("x is divisible by 2");
    } else {
        println!("x is not divisible by 4, 3, or 2");
    }


    // using if in a let statement
    let condition = true;
    let n = if condition {5} else {6};
    println!("The value of n is: {n}");
}
