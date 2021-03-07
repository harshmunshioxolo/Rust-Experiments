// Vectos are resizsable arrays

use std::mem;

pub fn run() {

    // Declating a vector
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    // Add onto vectors
    numbers.push(5);
    numbers.push(6);

    // Loop though the vector
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate

    for x in numbers.iter() {
        *x *=2;
    }
}