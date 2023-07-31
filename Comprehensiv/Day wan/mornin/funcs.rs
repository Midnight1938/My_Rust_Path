/*
 * The functions in rust are similar to C/C++.
 * But the last expression in the function is the return value, with a -> type annotation.
 
 ! Its also customary to use /// for documenting a function.
 */

fn main(){
    print_fzbz(10);
}

/// Determine whether the first argument is divisible by the second argument.
/// If the second argument is zero, the result is false.
fn divisibility(n: i32, div: i32) -> bool {
    if div == 0{
        return false;
    }
    n % div == 0
}

fn fixxbuzz(num: i32) -> String {
    let fizz = if divisibility(num, 3) {"Fizz"} else {""};
    let buzz = if divisibility(num, 5) {"Buzz"} else {""};
    if fizz.is_empty() && buzz.is_empty() {
        return format!("{num}");
    } 
    format!("{}{}", fizz, buzz)
}

fn print_fzbz(n: i32) {
    for i in 1..=n { // All numbers from 1 to n, =n means that it includet n
        println!("{}", fixxbuzz(i));
    }
}