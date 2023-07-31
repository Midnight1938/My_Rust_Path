
/*
 * We can use {:?} to print out an array, even pretty print it with {:#?}
 * But a traditional for loop is also available
 */

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix.iter() {
        println!("{:?}", row);
    }

}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut matrixx = [[0; 3]; 3];
    for i in 0..3{
        for j in 0..3{
            matrixx[j][i] = matrix[i][j];
        }
    }
    matrixx
}

fn main(){
    // We can also have the 2d matrix range hard coded with a matrix[3][3]
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("Current Matrix {:?}", matrix);
    
    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}