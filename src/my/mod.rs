
use super::foo;

pub fn call_foo() {
    foo::print();
}
pub fn print() {
    println!("my");
    f();
}

fn f() {
    println!("a");
}

pub mod a;