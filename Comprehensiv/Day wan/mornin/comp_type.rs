
/*
 * Compound Types
 * Arrays
 *  A value of the array type [T; N] holds N (a compile-time constant) elements of the same type T.
 * Tuples
 *  A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
 */

fn main(){
    // Array
    let a: [i8; 5] = [4, 2, 2, 3, 5]; // [type; size]
    let first = a[0];
    println!("The value of first point is: {}", first);
    println!("Array is {:?}", a); // Adding a # between : and ? will pretty print the debug output.
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Destructuring
    let (x, y, z) = tup;
    println!("The values in tup are: {}, {}, {}", x,y,z);
}