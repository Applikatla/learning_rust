
struct Device{
    id: String,
    status: bool
}

impl Device {
    fn enable(self: &mut Device) {
        self.status = true
    }
    fn new(id: String) -> Device {
        Device {
            id,
            status: false,
        }
    }
    fn print_info(&self) {
        println!("Device ID: {}", self.id);
        println!("Status: {}", self.status);
    }
}

fn main() {
    let mut d = Device{
        id: String::from("123"),
        status: false
    };
    println!("{}", d.id);
    println!("{}", d.status);

    d.enable();
    println!("{}", d.id);
    println!("{}", d.status);

    let id = String::from("device-1");
    let d = Device::new(id);
    println!("{}", d.id);
    println!("{}", d.status);

    d.print_info();
}
