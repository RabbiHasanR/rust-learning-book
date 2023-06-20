use std::io;

fn main() {

    loop {
        println!("Input number for fibonacci count:");
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Invalid input!");

        let n: i32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if n < 0 {
            println!("Input can not be negative number!");
            continue;
        }

        let result = fibonacci(n);
        println!("Fibonacci count is: {result}");
        break;
    }
    

}


fn fibonacci(n: i32) -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 1;
    if n == 1 {
        a
    } else {
        let mut index: i32 = 2;
        while index < n + 1 {
            let temp = a + b;
            a = b;
            b = temp;
            index += 1;
        }

        b
    }
}