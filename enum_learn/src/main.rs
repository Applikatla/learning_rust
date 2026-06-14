use crate::Option::Some;

#[derive(Debug)]
enum DeviceStatus {
    Online,
    Offline,
}

enum Message {
    Text(String),
    Number(i32),
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    OK(T),
    Err(E),
}

fn main() {
    let status = DeviceStatus::Online;
    println!("{:?}", status);
    println!("created status");
    let status1 = DeviceStatus::Offline;
    println!("{:?}", status1);
    match status {
        // DeviceStatus::Online => println!("device online"),
        DeviceStatus::Offline => println!("device offline"),
        DeviceStatus::Online => {
            println!("inside online block");
            if 1 == 1 {
                println!("1")
            }
        }
    }

    let text = Message::Text(String::from("hello"));
    let n = Message::Number(5);

    match text {
        Message::Text(msg) => println!("{}", msg),
        Message::Number(n) => println!("number: {}", n),
    }

    // match text {  // here it will give error becaue text owner ship is moved to msg in first match
    //     Message::Text(msg) => println!("{}",msg),  //iff this can happen only if we barrow text IN first match &text
    //     Message::Number(n) => println!("number: {}", n),
    // }

    // let mut text = Message::Text(String::from("hello"));
    // let mut n = Message::Number(5);
    // match &mut text {
    //     Message::Text(msg) => println!("{}",msg),
    //     Message::Number(n) => println!("number: {}", n),
    // }
    match n {
        Message::Number(num) => println!("{}", num),
        Message::Text(msg) => println!("{}", msg),
    }

    let some = Option::Some(String::from("some thing"));
    match some {
        Option::Some(s) => println!("{}", s),
        Option::None => println!("hey its None"),
    }

    let some: Option<String> = Option::None;
    match some {
        Option::None => println!("its None"),
        Option::Some(s) => println!("{}", s),
    }

    let user: Result<String, ()> = Result::OK(String::from("keshav"));
    match user {
        Result::OK(ok) => println!("{}", ok),
        Result::Err(err) => println!("{:?}", err),
    }

    let err: Result<(), String> = Result::Err(String::from("error"));
    match err {
        Result::Err(err) => println!("{}", err),
        Result::OK(ok) => println!("{:?}", ok),
    }

    let mut name = Option::Some(String::from("Hello"));
    if let Some(value) = &mut name {
        value.push_str(" world")
    }
    match name {
        Option::Some(x) => println!("{}", x),
        Option::None => {},
    }
}
