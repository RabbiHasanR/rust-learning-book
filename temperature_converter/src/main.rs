use std::io;

fn main() {
    println!("Input Temperature:");
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f32 =  temp.trim().parse().expect("You need to input valid number");

    println!("Input Convert To(For farhenhite F or Celcius C):");
    let mut convert_type = String::new();
    io::stdin()
        .read_line(&mut convert_type)
        .expect("Failed to read line");

    let convert_type: char = convert_type.trim().parse().expect("You need to enter F or C");

    if convert_type == 'F' || convert_type == 'f' || convert_type == 'C' || convert_type == 'c' {
        let result = temperature_converter(temp, convert_type);
        println!("Result is: {result}");
    } else {
        println!("You must input F or C");
    }

}


fn temperature_converter(temp: f32, convert_type: char) -> f32 {
    const T: f32 = 32.0;
    const FRACTION: f32 = 0.6;

    let c: f32 = (temp - T) * FRACTION;
    let f: f32 = (temp * FRACTION) + T;
    if convert_type == 'F' || convert_type == 'f' {
        f
    } else if convert_type == 'C' || convert_type == 'c' {
        c
    } else {
        0.0
    }
}
