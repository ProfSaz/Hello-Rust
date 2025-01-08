#![allow(unused)]

fn main() {
    //Strings are of two types Strings with capital S and &str
    // String- vector of u8 (Vec<u8>) valid UTF-8 used when data needs to be mutated or owned
    //&str - slice of u8(&[u8]) valid UTF-8 used for read only

    let msg: String = String::from("Hello rust");
    let len: usize = msg.len();
    println!("msg: {}", msg);
    println!("len {}", len);

    let msg: String = String::from("Hello rust");
    let s: &str = &msg[0..5];
    let s_len: usize = s.len();

    println!("s: {}", s);
    println!("len {}", s_len);

    let hello = "Hello Rust";

    let s = r#"
{ "a" : 1, 
  "b" : {"c" : 2},
  "d" : 3
}
"#;
    println!("s: {}", s);

    //Deref coercion
    let msg: String = String::from("Hello rust");
    let s = &msg;

    let mut msg: String = "Hello Rust".to_string();
    msg += "!";

    println!("msg: {}", msg);

    //String interpolation

    let lang = "Rust";
    let emoji = "happy";
    let msg = format!("Hello {lang} {emoji}");
    println!("{msg}");
}
