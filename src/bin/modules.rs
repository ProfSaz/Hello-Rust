#![allow(unused)]

use hello::my;

fn main() {
    my::print();
    my::a::print();
    let s = my::a::S {
        id: 1,
        name: "saz".to_string()
    };

    my::call_foo();
    my::a::call_foo_in();

    println!("{:?}", s);
}