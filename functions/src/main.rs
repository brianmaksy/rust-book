fn main() {
    another_function(5);
    print_labeled_measurement(5,'h');
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1 // returns something. An expression. If add ;, it will become a statement and error.
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
    // https://users.rust-lang.org/t/newbie-why-macros-vs-functions/1012
    // https://www.geeksforgeeks.org/macros-vs-functions/
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
// // loops 
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {}", count);
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {}", remaining);
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {}", count);
// }


// fn main() {
//     for number in (1..4).rev() {
//         println!("{}!", number);
//     }
//     println!("LIFTOFF!!!");
// } [prints 3,2,1]