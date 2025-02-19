
    use super::super::foo;

    pub fn print() {
        println!("a");
    }
    
    #[derive(Debug)]
    pub struct S {
        pub id: u32, 
        pub name: String
    }

    pub fn call_foo_in() {
        foo::print();
    }


