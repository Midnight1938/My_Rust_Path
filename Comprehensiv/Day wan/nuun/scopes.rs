
/*
 * Scopes in rust are a block of code within curly braces.
 * shadowed scopes are created by using the same variable name within a block of code, like redeclaring a variable.
 */

 fn main() {
    let a = 10;
    println!("before: {a}");

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed/redeclared in inner scope: {a}");
    }

    println!("after: {a}");
}