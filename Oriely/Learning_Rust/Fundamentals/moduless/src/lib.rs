
// Public can be used within the crate and outside the crate
pub fn greeter(string: &str) {
    println!("Hello, {}!", string);
}

fn secret_greeter(string: &str) {
    println!("*whispers* Hello there, {}!", string);
}