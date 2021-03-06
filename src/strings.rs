// Primitive strings = Immutable fixed length string in the memory
// String = Growable, heal allowcated

pub fn run() {
    let mut hello = String::from("Hello");

    // Get length of the string

    println!("{}", hello.len());

    println!("{}", hello);

    // two methods for "appending" more characters
    // Push Char
    hello.push('Q');
    println!("{}", hello);

    // Push a string    
    hello.push_str("YOBO!");
    println!("{}", hello);

    // getting the capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Is empty
    println!("Is empty: {}", hello.is_empty());

    // does it contain a word?
    println!("Contains hello? : {}", hello.contains("ello"));

    // check more operations in the documentation
}