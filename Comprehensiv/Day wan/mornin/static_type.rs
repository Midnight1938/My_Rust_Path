
/*
 * Rust supports the following static types:
 * - signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
 * - unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
 * - floating point: f32, f64
 * - char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
 * - bool either true or false
 * - &str string slices like "Hello World!"
 *      - Raw string slices allowing escaped characters like r"Hello \n World!"
 *     - Byte string slices like b"Hello World!" for u8 slices. U8 means unsigned 8-bit integer.
 * underscore can be used to improve readability, e.g. 1_000 is the same as 1000, and 0.000_001 is the same as 0.000001.
 
 ! You dont HAVE to define variable types, compiler will handle it for you. But you can if you want to.
 */

// Show all types
fn main(){
    let x: i8 = 5;
    println!("The value of signed:\t {}", x);
    let x = -5;
    println!("The value of unsign:\t {}", x);
    let x: f32 = 5.0;
    println!("The value of float:\t {}", x);
    
    let x = 'a';
    println!("The value of char:\t {}", x);
    
    let x = true;
    println!("The value of bool:\t {}", x);

    println!(r#"<a href="link.html">link</a>"#);
    println!("{:?}", b"bytes");
}