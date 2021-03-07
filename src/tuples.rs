// Tuples group values of different types
// Also work on arrays within the same code

pub fn run() {
    let person: (&str, &str, i8) = ("Harsh", "Munshi", 29);
    println("My name is {} {} and I am {} years old!", person.0, person.1, person.2);

    // Array has fixed arays
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", numbers);

    // NOTE: In case you want to get slices of the numbers
    let slice: &[i32] = &numbers[1..2];
    println!("Slice: [:?]", slice);
}