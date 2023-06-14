// Computational Physics exercise 5
// Jan Kesting, Felix Fleischle - Group 1 Jeong Yun Choi

// We decided to do this exercise in Rust - for fun :)
// In case you do not want to look at the Rust code, we have a solution in python which is appended to the pdf at the end
// The output of the rust code - and thus the solutions - are printed at the end of the code

use::std::vec;

// This is a function performing the gaussian elimination. The function borrows the matrix M and the vector y as a mutable reference
fn gaussian_elimination(M: &mut Vec<Vec<f64>>, y: &mut Vec<f64> ) {
    // First, transform the matrix M into an upper triangle using elementary operations, and apply the same operations to y
    for i in 0..(y.len()-1) {
        let factor: f64 = M[i+1][i] / M[i][i];
        M[i+1][i+1] -= factor * M[i][i+1];
        y[i+1] -= factor * y[i];
        M[i+1][i] = 0.;
    }
    
    // Next, transform into a diagonal matrix
    for j in (1..=y.len()-1).rev() {
        let factor = M[j-1][j] / M[j][j];
        y[j-1] -= factor * y[j];
        M[j-1][j] = 0.;
    }

    // Finally, create the unitary matrix, after which y is equal to the solution vector x
    for k in 0..y.len() {
        y[k] = y[k] / M[k][k];
        M[k][k] = 1.;
    }
}

// This function creates a tridiagonal matrix from the values N, a, b and c
fn construct_matrix(M: &mut Vec<Vec<f64>>, N: i32, a: f64, b:f64, c:f64) {
    for i in 0..N {
        let mut row: Vec<f64> = Vec::new();
        if i == 0 {
            row.push(b);
            row.push(c);
            for j in 0..N-2 {
                row.push(0.);
            }
        } else if i == N-1 {
            for j in 0..N-2 {
                row.push(0.);
            }
            row.push(a);
            row.push(b);
        } else {
            for j in 0..i-1 {
                row.push(0.);
            }
            row.push(a);
            row.push(b);
            row.push(c);
            for j in i+2..N {
                row.push(0.);
            }
        }
        M.push(row);
    }
}

// A function doing the backwards substitution
// This function takes the original matrix M and the solution vector x, and returns y
fn backwards_substitution(M: & Vec<Vec<f64>>, x: & Vec<f64>) -> Vec<f64> {
    let mut y: Vec<f64> = Vec::new();

    for i in 0..x.len() {
        let mut sum = 0.;
        for j in 0..x.len() {
            sum += M[i][j] * x[j] ;
        }
        y.push(sum);
    }
    return y;
}

fn main() {
    // First, create the Matrix M using our predefined function
    let mut M: Vec<Vec<f64>> = Vec::new();
    let N = 10;
    let a = -1.;
    let b = 1.5;
    let c = -1.;
    let mut y: Vec<f64> = vec![0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1];

    construct_matrix(&mut M, N, a, b, c);

    // Print M to see what it looks like
    println!("Printing M:");
    for i in 0..y.len() {
        for j in 0..y.len() {
            print!("{} ", M[i][j]);
        }
        println!("");
    }

    // Perform the gaussian elimination
    gaussian_elimination(&mut M, &mut y);

    // Printing y at the end, which is equal to the solution vector x
    println!("Printing yend:");
    for i in 0..y.len() {
        println!("{}", y[i]);
    }

    // Printing the matrix at the end to confirm that the algorithm did the correct thing
    println!("Printing M_end:");
    for i in 0..y.len() {
        for j in 0..y.len() {
            print!("{} ", M[i][j]);
        }
    println!("");
    }

    // Do a backwards substitution

    // First, create the matrix M again
    let mut M_copy: Vec<Vec<f64>> = Vec::new();
    construct_matrix(&mut M_copy, N, a, b, c);

    // perform the backwards substitution using our defined function
    let y_backwards = backwards_substitution(&M_copy, &y);

    // Print y_backwards
    println!("Printing y_backwards:");
    for i in 0..y_backwards.len() {
        println!("{}", y_backwards[i]);
    }

    // Printing the deviation between y at the start and y_backwards
    println!("Printing deviation:");
    for i in 0..y_backwards.len() {
        println!("{:+e}", y_backwards[i] - 0.1);
    }

}
