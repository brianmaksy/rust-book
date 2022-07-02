fn main() {
    // let mut x = 6
    let x = 6;
    println!("The value of x is: {}", x);
    // x = 7;
    // println!("The value of x is: {}", x);

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let x = x + 1; // shadowing ok for immutable vars. Also, as if creating a new var with 'let'. 

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x); // 14 
    }

    println!("The value of x is: {}", x); //7 

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup; // add underscore in front of var if not used later (?) 

    println!("The value of y is: {}", y);

    // Unlike a tuple, every element of an array must have the same type --> like in python. Array same type, list = diff type (in Py)
    // also, fixed length. Vector = variable length. 

    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    let _b = [3; 5]; // [3, 3, 3, 3, 3]
    a[2] = 5;
    println!("{}", a[2]);

}