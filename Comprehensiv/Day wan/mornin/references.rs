
/*
 * Dangling References in rust are not allowed
 * But borrowing a variable and then using it in the same scope is allowed
 */


 fn main() {
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");
}

/*
 ? This isnt allowed, and the braces are scopes
fn notAllow() {
    let ref_x: &i32;
    {
        let x: i32 = 10;
        ref_x = &x;
    }
    println!("ref_x: {ref_x}");
}
*/