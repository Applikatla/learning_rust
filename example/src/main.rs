fn print_length(s: &String) {
    println!("Length = {}", s.len());
}

fn append_world(s: &mut String) {
    s.push_str(" world");
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn take_and_return(mut s: String) -> String {
    s.push_str(" brother");
    s
}

fn add_suffix(mut c: String) -> String {
    c.push_str(" blockchain");
    c
}

fn main() {
    let s = String::from("hello");

    print_length(&s);

    println!("{}", s); // I want this to work

    let mut x = String::from("hello");
    append_world(&mut x);
    println!("{}", x);

    let len = get_length(&x);
    println!("{}", len);

    let r1 = &x;
    println!("{}", r1);
    let r2 = &mut x;
    r2.push_str(" keshav");
    println!("{}", r2);

    let x = take_and_return(x);

    println!("{}", x);

    let company = String::from("Venziq");
    let company = add_suffix(company);
    println!("{}", company);
}
