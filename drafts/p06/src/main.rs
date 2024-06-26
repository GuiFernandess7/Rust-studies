use std::collections::HashMap;

struct Customer {
    name: String,
    address: String,
    balance: f32,
}

struct Rectangle<T, U> {
    length: T,
    height: U,
}

trait Shape {
    fn new(length: f32, width: f32) -> Self;
    fn area(&self) -> f32;
}

fn main() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");

    for (k, v) in heroes.iter(){
        println!("{} = {}", k, v);
    }

    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }

    let mut bob = Customer{
        name: String::from("Bob smith"),
        address: String::from("555 Main St"),
        balance: 234.50,
    };

    bob.address = String::from("505 Main St");

    let rec = Rectangle {length: 4, height: 10.5};
}
