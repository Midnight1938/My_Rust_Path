
/*
 * Rust does not apply implicit type conversion to user-defined types.
 */

fn multip(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 8;
    let y: i16 = 2;

    // ! This will not compile
    //multip(x, y); 

    // Hence we convert the types explicitly
    let ans = multip(x.into(), y); // into is a trait that converts i8 to i16
    println!("{x} times {y} is {ans}");
}
