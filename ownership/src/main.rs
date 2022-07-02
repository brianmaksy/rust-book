fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`    
}

// Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. --> the "drop" function. 