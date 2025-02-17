fn main() {
    // tup
    let x = (1, 2, "a");
    println!("X[0] = {}",x.0);
    println!("Hello, world!");
    //arr
    let arr = [1, 2, 3, 4, 5];
    println!("Arr 1st element: {}", arr[0]);
    // vectors
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("Vector 1st element: {}", vec[0]);
    // creating a empty vector
    let mut vec1 = Vec::new();
    vec1.push(2);
    vec1.push(23);
    vec1.push(234);
    println!("Vector: {:?}", vec);
    println!("Vector: {:?}", vec1);
    //slicing
    let mut slice = &vec[1..3];
    println!("{:?}", slice); // Elements at indices 2, 3, and 4 (i.e., [30, 40, 50])
    slice = &arr[1..4]; // Elements at indices 1, 2, and 3 (i.e., [2, 3, 4])
    println!("{:?}", slice);
    let s = "hello"; 
    // // or
    // let s = String::from("hello");
    let slice = &s[0..3]; // Valid, slices the first three characters ("hel")
    println!("{}", slice); // Output: "hel"
    let mut vec = vec![1, 2, 3, 4, 5];

    let slice = &mut vec[2..4]; // Mutable slice to modify elements at indices 2 and 3
    slice[0] = 10;  // Modify element at index 2
    slice[1] = 20;  // Modify element at index 3

    println!("{:?}", vec); // Output: [1, 2, 10, 20, 5]

}
