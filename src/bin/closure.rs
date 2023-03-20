fn main() {
    let greeting = String::from("Hello");
    let closure = |name| println!("{greeting}, {name}!");
    
    closure("world");
    closure("Bob");
    
    println!("{greeting}, everyone!");
}