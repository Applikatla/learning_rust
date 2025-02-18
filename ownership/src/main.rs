fn main() {
    println!("Hello, world!");
    let s1 = String::from("Hello");
    let s2 = s1; // Ownership moves from s1 to s2

    println!("{}", s2); // ✅ Works
    // println!("{}", s1); // ❌ ERROR! `s1` is moved

    // Cloning to Keep Both Variables
    let x1 = String::from("case 2");
    let x2 = x1.clone(); // Deep copy

    println!("{x1}"); // ✅ Works because `clone()` creates a separate copy
    println!("{x2}");
    //  Ownership with Functions
    let _y = String::from("case 3");
    take_owner_ship(_y);
    // println!("{}", s); // ❌ ERROR! `s` is moved
    //Returning Ownership
    let z = String::from("case 4");
    let _z1 = take_and_give(z);
    println!("{_z1}");  // ✅ Works

    // Mutable Borrowing
    let mut a = String::from("Hi");
    change(&mut a);
    println!("{a}");

}

fn take_owner_ship(_y:String) {
    println!("{_y}")
}

fn take_and_give(_z:String) -> String {
    return _z; // Returns ownership
} 

fn change(_a: &mut String) {
    _a.push_str(" and hello");
}
