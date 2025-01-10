#![allow(unused)]


fn main() {
//  *, +, - , / 
    let a: i32 = 2;
    let b: i32 = 4;
    let c = a - b; 
    let c = a * b; 
    let c = a / b;
    println!("{a} / {b} = {c}");

// % (remaninder != mod operator)
let a = -1; 
let b = 2;
let rem = a % b; 
println!("{a} % {b} = {rem}");

// Literals 
let a = 1i32;
let b = 3u64;
let c = 1.23e3;
let d = 1_000_000_000u32; 

//Boolean 
let a = true && false; 
let a = true || false;
let a = !true;

//Bitwise 
let a: u8 = 5;
let b: u8 = 3; 
println!("a & b = {:03b}", a & b);
println!("a & b = {:03b}", a | b); 
println!("a & b = {:03b}", a ^  b); 
println!("!a = {:03b}", !a); 
println!("1 << 3 = {:03b}", 1u32 << 3); 
println!("10 >> 2 = {}", 10u32 >> 2);

}