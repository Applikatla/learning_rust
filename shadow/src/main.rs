fn main() {
    let x = 10;
    let x = x + 1;
    println!("X = {}", x);
    {
        let x = x * 2;
        println!("X in another sope = {}", x);

    }
    let str = "  ";
    println!("lenght of str: {}", str.len());
    println!("Hello, world!");
}
