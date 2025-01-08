#![allow(unused)]

fn main() {
    //variables in rust by default are immutable, but if you want to declare a mutable variable you use the mut key word

    let x: i32 = -23;
    // This will not compile
    // x += 1

    let mut y: i32 = 12;
    // This will compile
    y += 2;
    println!("{}", y);

    // you can also redeclare a variable this is called shadowing in rust

    let x = "yes";
    println!("{}", x);

    let x = true;
    println!("{}", x);
}
