pub fn run() {
    // Print to console
    println!("Hello from the printRS file");

    // Basic formatting
    println!("Number: {}", 1);

    println!("{} is from {}", "Harsh", "Bonn");

    // Positional params
    println!("{0} is from {1} and {0} like to {2}", "Harsh", "Ahmeadabad", "Code");

    // Named arguments
    println!("{name} is from {place}", name="Harsh", place="Bonn");

    //Plceholder for debug
    println!("{:?}", (12, true, "hello"));
}