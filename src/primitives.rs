// Primitives
// Int: u8, i8, u16, i16, u32...64..128
// Floats f32, f64
// Boolean
// Char
// Tuples
// Arrays

// Rust is a statistically typed index, must know alll variables

pub fn run() {
    // Defaults to "i32"
    let x = 1;

    // Floats defaults to "f64"
    let y = 2.5;

    // For specific types
    // let var:<type> = decl
    // example int of a specific type
    let p: u8 = 253;

    // find the maximum type
    println!("Max i8 = {}", std::i8::MAX);

    // some boolean
    let someval = true;

    // getting boolean from an expression
    let is_greater = 10 > 5;

    println!("The output of evaluation is: {}", is_greater);

    // char type, this can only hold only one chatacter
    let a1 = 'a';

    //emojis
    let face = '\u{1F600}';
    println!("I feel {}", face);
}