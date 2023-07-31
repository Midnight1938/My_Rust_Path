
/*
 * Methods in rust are functions that are defined in the context of a struct
 * Classes are not a thing in rust, so methods with impl are the closest thing
 * Overloading isnt supported, but the parameters can be generic
 */

struct Rectangle {
    width: f32,
    height: f32,
}

// Impl is short for implementation
// and is used to define methods that belong to a struct is like a class
impl Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
    fn inc_width(&mut self, delta: f32) {
        self.width += delta;
    }
}

fn main() {
    let mut rect = Rectangle  {width: 10.0, height: 20.0};
    println!("Area: {}", rect.area());
    rect.inc_width(10.0);
    println!("Area: {}", rect.area());
}