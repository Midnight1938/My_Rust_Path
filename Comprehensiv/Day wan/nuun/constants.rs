
/*
 * Constants in rust are like those seen in C++.
 * The variables are evaluated at compile time and are immutable by default.

 * Static variables are also immutable by default, but they are evaluated at runtime.

 ! Properties table:
 *  | Property	                        | Static	            | Constant      |
 *  | Has an address in memory	        | Yes	                | No (inlined)  |
 *  | Lives for the entirety of program	| Yes	                | No            |
 *  | Can be mutable	                | Yes (unsafe)	        | No            |
 *  | Evaluated at compile time	        | Yes (init at compile) | Yes           |
 *  | Inlined wherever it is used	    | No	                | Yes           |
 */

const MAX_POINTS: u32 = 100_000;
static MAX_POINTS_STATIC: u32 = 100_000;

fn main(){
    println!("MAX_POINTS: {}", MAX_POINTS);
    println!("MAX_POINTS_STATIC: {}", MAX_POINTS_STATIC);
    

}