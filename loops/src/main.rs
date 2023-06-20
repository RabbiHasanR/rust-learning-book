fn main() {
    // loop {
    //     println!("Hello, world!");
    // }


    // returning values from loop

    println!("Loop examples");
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    // nested loops
    println!("Nested Loop examples");
    let mut count = 0;

    'counting_up: loop {
        println!("Count: {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining: {remaining}");

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

    println!("End count: {count}");


    // while loop
    println!("while loop example!");
    let mut number = 3;

    while number != 0 {
        println!("Number: {number}");

        number -= 1;
    }

    println!("LIFTOFF");


    // array item looping with while

    let a = ["1", "2", "3", "4"];
    let mut index = 0;

    while index < 4 {
        println!("A value is : {}", a[index]);
        index += 1;
    }



    // for loop
    println!("for loop example!");
    let numbers = [1,2,3,4,5,6,7,8,9,10];

    for element in numbers {
        println!("element is: {element}");
    }
   

   // range for loop example and rev example

   println!("Range for loop example and rev example!");

   for number in (1..4).rev() {
    println!("Number : {number}");
   }

   println!("LIFTOFF!!!");
}
