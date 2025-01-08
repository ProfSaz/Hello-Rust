#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}


// Struct Methods
impl Point {

    // Associated functions - static methods
    fn zero() -> Self {
        Self {x: 0.0, y: 0.0}
    }

    // Method 

    fn move_to (&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn dist(&self) -> f32 {
        (self.x * self.x + self.y * self.y ).sqrt()
    }


}

// Methods

fn main() {
    let mut p = Point::zero();
    p.move_to(2.0, 1.0);
    println!("{:?}", p );

    let d = p.dist();
    println!("distance {:?}", d);

}
