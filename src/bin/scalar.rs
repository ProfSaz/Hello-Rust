#![allow(unused)]

use std::{char, u32};
fn main() { 

    //Integers
    // signed integers ranges from 2**(n-1) to 2**(n-1)-1
    //starts from i8 to i128
    let i8: i8 = 4; 
    let i16: i16 = 3300; 
    // unsinged integers ranges from 0 to 2**n-1 that is unsigned integers can never be negative just like in solidity 
    //starts from u8 to u128
    let u8: u8 = 255;

    //Floats 
    //These are used for precision and has f32 and f64 range only 
    let f0 = 0.01;
    let f1: f32 = 0.01; 

    //Boolean 
    let truth = true;

    //Characters 
    let c: char = 'c';
    // when a single quote is used its a character when double quoutes are used its a string

    //Type conversion 
     let i: i32 = 1;
     let u: u32 = i as u32; 
     let x: u32 = u + (i as u32);

     //Min and max 
     //Integer, floats and Characters has min and max value but boolean doesnt 

     let min_i: i32 = i32::MIN;
     let max_i: i32 = i32::MAX;

     println!("{min_i}");
     println!("{max_i}");

     let min_char: char = char::MIN;
     let max_char: char = char::MAX;

     println!("{min_char}");
     println!("{max_char}");

     //Overflow 
     // when you run this it overflows and panic but when you use the --release tag on the terminal it actually calculates to zero and adds the remainder to start a count again 

    
    //  let mut u: u32 = u32::MAX;
    //  u += 2; 
    //  println!("overflow {u}");

    //the checked_add tag returns a Some(number) when there is no overflow and returns None when there is an overflow, while the wrapped_add actually does what the release tag do it wraps the number when there is an overflow and starts from 0 
     let u = u32::checked_add(u32::MAX, 1);
    println!(" check add{:?}", u);

     let u = u32::wrapping_add(u32::MAX, 1);
     println!(" wrapp add {:?}", u);




}