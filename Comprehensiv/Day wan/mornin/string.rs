
/*
 * Theres a difference between a string and a &str (string slice).
 * A string is a heap-allocated string.
 * whereas a string slice is just a reference to some UTF-8 bytes stored elsewhere.
 ? &str an immutable reference to a string slice.
 ? String a mutable string buffer. 

 * String::from() creates a string from a string literal; 
 * String::new() creates a new empty string, to which string data can be added using the push() and push_str() methods.
 */

fn main(){
    let s1: &str = "Warudo"; // Basic Slice
    println!("s1: {s1}");

    let mut s2: String = String::from("Henlo "); // Normal String
    println!("s2: {s2}");
    s2.push_str(s1); // Pushing a slice into a string
    println!("s2: {s2}");

    let s3: &str = &s2[..6]; // Slicing a string out till the 6th byte
    println!("s3: {s3}");
}