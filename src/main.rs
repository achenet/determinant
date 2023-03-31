use std::io;
// Caluculate the determinant of a matrix
// TODO have the determinant functions return an
// Option or Result instead of just an integer
// TODO generalize to f32/f64 instead of just integers
// A matrix is a vector of rows vectors.
// First coordinate is the row, second coordinate is the vector

mod tests;

fn main() {
    let a = get_input_matrix();
    println!("your matrix:");
    pretty_print(a.clone());
    println!("the determinant of you matrix is {}", determinant_cramer(a));
}

#[allow(non_snake_case)]
#[allow(dead_code)]
fn determinant_bezout(A: Vec<Vec<i32>>) -> i32 {
    if !is_a_square_matrix(A.clone()) {
        return 0;
    }
    let n = A.len();
    if n == 1 {
        return A[0][0];
    }
    if n == 2 {
        return A[0][0] * A[1][1] - A[0][1] * A[1][0];
    }
    // TODO
    7
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn extract_submatrix(A: Vec<Vec<i32>>, i: usize, j: usize) -> Vec<Vec<i32>> {
    vec![]
}

#[allow(non_snake_case)]
fn determinant_cramer(A: Vec<Vec<i32>>) -> i32 {
    if !is_a_square_matrix(A.clone()) {
        return 0;
    }
    let n = A.len();

    // Sum over all permuations s in Sn sign(s)*a_s(1)1 ... a_s(n)n
    let mut sum = 0;
    for s in generate_permuations(n) {
        let mut product = 1;
        for i in 0..n {
            product = product * A[s[i]][i]
        }
        sum += sign(s) * product;
    }
    sum
}

// Use Heap's algorithm :)
fn generate_permuations(n: usize) -> Vec<Vec<usize>> {
    let mut out: Vec<Vec<usize>> = vec![];
    let a = (0..n).collect();
    gen(n, a, &mut out);
    out
}

fn gen(n: usize, a: Vec<usize>, out: &mut Vec<Vec<usize>>) {
    if n == 1 {
        out.push(a.clone());
        return;
    }
    gen(n - 1, a.clone(), out);
    for i in 0..(n - 1) {
        let b = swap(i, n - 1, a.clone());
        gen(n - 1, b, out);
    }
}

fn swap(i: usize, j: usize, mut a: Vec<usize>) -> Vec<usize> {
    let t = a[i];
    a[i] = a[j];
    a[j] = t;
    a
}

#[allow(non_snake_case)]
fn is_a_square_matrix(A: Vec<Vec<i32>>) -> bool {
    let n = A.len();
    for col in A {
        if col.len() != n {
            return false;
        }
    }
    true
}

fn sign(s: Vec<usize>) -> i32 {
    let mut inversions = 0;
    for (i, val) in s.iter().enumerate() {
        if val > &i {
            inversions += 1;
        }
    }
    let base: i32 = -1;
    base.pow(inversions)
}

fn get_input_matrix() -> Vec<Vec<i32>> {
    let mut input = String::new();
    println!("How many rows does the square matrix have?");
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();

    let mut a: Vec<Vec<i32>> = vec![];
    for i in 0..n {
        println!("please input row {}", i);
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.parse::<i32>().expect("failed to parse int"))
            .collect();
        a.push(row);
        input.clear();
    }
    a
}

fn pretty_print(matrix: Vec<Vec<i32>>) {
    for row in matrix {
        print!("| ");
        for col in row {
            print!("{} ", col);
        }
        println!("|");
    }
}
