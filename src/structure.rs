// In this code we will understand how to work with structures

struct Point {
    x: u8,
    y: u8,
}

struct Person {
    firstname: String,
    lastname: String,
}

// some function with Person
impl Person {
    //construct a person
    fn new(first: &str, last: &str) -> Person{
        Person{
            firstname: first.to_string(),
            lastname: last.to_string()
        }
    }

    // full name
    fn full_name(&self)  -> String{
        format!("{} {}", self.firstname, self.lastname)
    }

    // set last name
    fn set_last_name (&mut self, last: &str) {
        self.lastname = last.to_string();
    }
}

pub fn run(){
    let mut p = Point{x:5, y:3};
    println!("Current coordinates are x: {}, y: {}", p.x, p.y);

    let mut per = Person::new("Harsh", "Munshi");
    println!("Person {}", per.full_name());

    per.set_last_name("Gunshi");
    println!("Person {} {}", per.firstname, per.lastname);
}