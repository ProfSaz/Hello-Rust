#![allow(unused)]

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
    Hex(String),
    Hsl { h: u8, s: u8, l: u8 },
}
fn main() {
    //Enum: these are used to determine a finite state
    let color: Color = Color::Red;
    let color = Color::Green;
    let color = Color::Rgba(0, 0, 255, 0.5);
    let color = Color::Hex("#ffffff".to_string());
    let color = Color::Hsl { h: 0, s: 1, l: 200 };
    //Attribute - Debu and partialEq
    println!("{:?}", color);
    //PartialEq: compares two enums
    println!("{}", Color::Green == Color::Red);
    println!("{}", Color::Green == Color::Green);
    //Option is an enum that can take on two values = Some(11) | None

    let x: Option<i32> = None;
    let x: Option<i32> = Some(-11);
    println!("option:{:?}", x);
    //Result = Ok(10) | Err("div by 0 ");
    let res: Result<u32, String> = Ok(5);
    let res: Result<u32, String> = Err("div by 0".to_string());

    println!("result: {:?}", res);
}
