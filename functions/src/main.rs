fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(100,'h');

    let y = {
        let x = 5; // this is statement
        x + 1 // this is expression
    };

    println!("The value of y is: {y}");

    let f =  five();

    println!("The value of f is: {f}");

    let p = plus_one(100);

    println!("The value of p is: {p}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}



// return function

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
