pub fn run() {
    greet("Harsh", "Munshi");

    // we can also define something like lambda funcion
    // in rust it is called closure

    let add_sums = |n1:i32, n2:i32| n1 + n2;
    println!("C SUM: {}", add_sums(5,3));
}


// defining a function
fn greet(greet: &str, name: &str) {
    println!("Hello {} {} nice ot meet you!", greet, name);
}