// creating variables in rust

pub fn run() {
    let name = "harsh";

    // this will spawn a "const" variable
    // let make it mutable
    let mut age = 29;
    println!("My name is {name} and I am {age}", name=name, age=age);
    age = 30;
    println!("My name is {name} and I am {age}", name=name, age=age);

    // we can also use the keyword const but for that we have to add a file.
    // But for that we need to also specify the data type

    const ID: i32 = 001;
    println!("ID: {id}", id=ID);

}