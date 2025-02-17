fn main() {
    let condition = true;
    let x = if condition {5} else {6};
    // // but can't 
    // let x = if condition {5} else {"six"};
    println!("Value of x is {x}");
    println!("Hello, world!");
    // // continues loop like while true
    // loop {
    //     println!("{x}");
    // }
    // assigning value to loop
    let mut counter = 0;
    let mut c = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break {
                counter = counter * 2;
                c = c + 2;
                (counter, c)
            };
        }
    };
    println!("The result is: {}", result.0);
    // for loop 

    for number in 1..4 {
        println!("{number}!");
    }

    // reverse for loop
    println!("..........");
    for number in (1..4).rev() {
        println!("{number}!");
    }

}
