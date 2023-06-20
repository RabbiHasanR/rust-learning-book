use std::io;


fn main() {
    // #Scaler type:
    // Integer type : signd(i8, i32, i64, isize) and unsignd(u8, u32, u64, usize)
    println!("Scaler types:");
    println!("Integer types:");
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");

    //floating point type: f32, f64 . all floating points number are signed
    println!("floating point types:");
    let x = 2.0;
    let y: f32 = 3.0;

    println!("{x}");
    println!("{y}");


    // numberic operations
    println!("numberic operations:");
    let sum = 2 + 3;
    println!("sum: {sum}");
    let difference = 10 - 20;
    println!("difference: {difference}");
    let product = 3 * 6;
    println!("product: {product}");

    let quotient = 56.7 / 32.2;
    println!("quotient: {quotient}");
    let truncate = -5 / 3;
    println!("truncate: {truncate}");

    let reminder = 43 % 6;
    println!("reminder: {reminder}");


    // the boolean type
    println!("boolean types:");
    let t = true;
    let f = false;

    println!("{t}");
    println!("{f}");


    // character type:
    println!("character types:");
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    println!("{c}");
    println!("{z}");
    println!("{heart_eyed_cat}");




    // #Compound types:
    // The tuple type: : fixed length
    println!("Compound types:");
    println!("Tuple type:");
    let tup: (i32, f32, u8) = (500, 6.4, 1);

    let (i, j, k) = tup;

    println!("The value of i is: {i}");
    println!("The value of j is: {j}");
    println!("The value of k is: {k}");

    // get tuple by index:
    println!("get tuple by index:");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");

    // empty tuple

    let etup = ();
    // println!("empty tuple: {etup}");

    // Array type: fixed length and same data type:
    println!("Array type:");

    let array = [1,2,3,4,5];

    // println!("Array is: {array}")

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // accessing array elements:

    let first = months[0];
    let second = months[1];
    println!("{first}");
    println!("{second}");



    // invalid element array access

    let a = [1,2,3,4,5,6,7,8,9,10];

    println!("Please enter array index!");

    let mut index =String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Faild to read index!");

    let index: usize = index.trim().parse().expect("Index entered was not number!");
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");




}
