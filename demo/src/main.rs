fn main() {
    println!("Hello, world!");
    let s = String::from("Hello");
    let byte = s.as_bytes();

    for (i, &items) in byte.iter().enumerate() {
        println!("item {i}: {items}"); 
    }
    for (i, items) in s.chars().enumerate() {
        println!("item {}: {}", i, items);
    }
}
