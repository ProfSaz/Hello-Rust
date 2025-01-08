#![allow(unused)]

// Compound data types
// - tuple
// - array

fn main() {
    //Tuple
    let t: (bool, u32, char) = (true, 1, 'c');
    println!("t = {}, {}, {}", t.0, t.1, t.2);

    //Destructure
    let (a, b, c) = t;

    // ignore with
    let (_, b, _) = t;

    //Empty tuple - unit type
    let t = ();

    //Nested tuple

    let nested = ((1.23, 'a'), (true, 1u32, 'b'), ());

    println!(
        "nested {} {} {} {}",
        nested.0 .0, nested.0 .1, nested.1 .0, nested.1 .1
    );

    //Arrays
    //Arrays are compound type whole length are known at compile time.
    let arr: [u32; 3] = [1, 2, 3];
    println!("arr 2: {}", arr[2]);

    let mut arr: [u32; 3] = [1, 2, 3];
    arr[1] = 9;

    println!("arr 2: {}", arr[1]);

    let arr: [u32; 10] = [0; 10];
    println!("arr {:?}", arr);

    //Slice
    //Size is unknown at compile time only at run time
    let nums: [i32; 10] = [-1, 1, -2, 2, -3, 3, -4, 4, -5, 5];

    let s = &nums[..3];
    println!("first 3: {:?}", s);

    let s = &nums[3..7];
    println!("middle 4: {:?}", s);

    let s = &nums[7..];
    println!("last 3: {:?}", s);

    let s = &nums[..];
    println!("all values: {:?}", s);
}
