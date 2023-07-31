
/*
 * Slices are similar to arrays but their size is not known at compile time.
 */

fn main(){
    let mut a = [0, 1, 2, 3, 4];
    let complete = &a[..]; // A slice containing all of the elements in a
    let middle = &a[1..4]; // A slice of a: only the elements 1, 2, and 3
    println!("The value of complete is: {:?}", complete);
    //! a[2] = 10;
    //? not allowed because `a` is borrowed as immutable
    println!("The value of middle is: {:?}", middle);
}