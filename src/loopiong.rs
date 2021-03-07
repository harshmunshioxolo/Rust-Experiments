

pub fn run() {
    // looping
    let mut count = 0;

    // Writing an infinite loop
    loop {

        count+=1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    }

    // Writing  while loop
    while count <= 100 {
        if count %15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
                println!("fizz");
        } else if count % 5 == 0 {
                println! ("Buzz");
        }
        count += 1;
    }

    // Using for loops
    for x in 10..100 {
        if x% 15 == 0 { println!("fizzbuzz");}
        else if x%3 == 0 {println!("fizz");}
        else if x%5 == 0 {println!("Buzz");}
        else {println!("{}", x);}
    }
}