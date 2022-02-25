enum IpAddrKind {
    V4,
    V6
}

struct IpAddr1 {
    kind: IpAddrKind,
    address: String
}

enum IpAddr2 {
    V4(String),
    V6(String)
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String)
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        // method body
    }
}

fn route(ip_kind: IpAddrKind) {
    // method body
}

fn main() {
   let four = IpAddrKind::V4;
   let six = IpAddrKind::V6;

   route(IpAddrKind::V4);
   route(IpAddrKind::V6);

   let home1 = IpAddr1 {
       kind: IpAddrKind::V4,
       address: String::from("127.0.0.1")
   };

   let loopback1 = IpAddr1 {
       kind: IpAddrKind::V6,
       address: String::from("::1")
   };

   let home2 = IpAddr2::V4(String::from("127.0.0.1"));
   let loopback2 = IpAddr2::V6(String::from("::1"));

   let home3 = IpAddr3::V4(127, 0 , 0, 1);
   let loopback3 = IpAddr3::V6(String::from("::1"));

   let m = Message::Write(String::from("hello"));
   m.call();

   // Option
   let some_number = Some(5); // = Option<i32>
   let some_string = Some("a string"); // = Option<&str>

   let absent_number: Option<i32> = None;

   let x: i8 = 5;
   let y: Option<i8> = Some(5);

   // doesn't work, first Option<T> needs to be converted to T
   // let sum = x + y;

   let sum = x + y.unwrap(); // unwraps the value of Some of Option<i8>
}
