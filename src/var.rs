// creating variables in rust

pub fn run() {
    let name = "harsh";

    // this will spawn a "const" variable
    // let make it mutable
    let mut age = 29;
    println!("My name is {name} and I am {age}", name=name, age=age);
    age = 30;
    println!("My name is {name} and I am {age}", name=name, age=age);
}