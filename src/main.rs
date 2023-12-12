use std::fs::read_to_string;
use nalgebra::{DMatrix, Matrix, VecStorage, Dyn};

type GalaxyMatrix = Matrix<String, Dyn, Dyn, VecStorage<String, Dyn, Dyn>>;

fn get_matrix(filename: &str) -> GalaxyMatrix {
    let contents = read_to_string(filename).expect("Something went wrong reading the file");
    let rows_list= contents.lines().collect::<Vec<&str>>();
    let rows = rows_list.len();
    let cols = rows_list[0].len();
    let flattened = rows_list.concat().chars().map(|x| x.to_string()).collect::<Vec<String>>();
    DMatrix::from_vec(rows, cols, flattened)
}

fn expand_matrix(matrix: GalaxyMatrix) -> GalaxyMatrix {
    let mut rows_to_insert = Vec::new();
    for (i, row) in matrix.row_iter().enumerate() {
        if row.iter().all(|x| x == ".") {
            rows_to_insert.push(i);
        }
    }
    // TODO this is not working
    for &row in rows_to_insert.iter().rev() {
        let matrix = matrix.clone();
        matrix.insert_rows(row + 1, 1, ".".to_owned());
    }
    matrix
}

fn solution(filename: &str) -> i32 {
    let matrix = expand_matrix(get_matrix(filename));
    println!("{}", matrix);
    0
}

fn main() {
    assert_eq!(solution("example.txt"), 374);
    assert_eq!(solution("input.txt"), 0); 
}
