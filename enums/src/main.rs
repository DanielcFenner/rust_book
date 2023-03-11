// You can only be 1 of a type within an enum
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // enums can have methods too
    }
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6("::1".to_string());
}
