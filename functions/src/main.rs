fn main() {
    println!("Hello, world!");
    new_fun(4);
    some_function();
    let x: i32;
    x = function_with_return();
    println!("Value of X is {x}");
}

fn new_fun(mut x:i128) {
    x = x + x;
    println!("Value of changed x: {x}");
}

fn some_function() {
    let _y = {
        let mut x = 3;
        x = x + 1;
        println!("Value of x is: {x}")
    };
}

// functions with return 

fn function_with_return() -> i32 {
    return 5;
}