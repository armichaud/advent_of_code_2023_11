use std::fs::read_to_string;
use nalgebra::{DMatrix, Matrix, VecStorage, Dyn};

type GalaxyMatrix = Matrix<String, Dyn, Dyn, VecStorage<String, Dyn, Dyn>>;

fn get_matrix(filename: &str) -> GalaxyMatrix {
    let contents = read_to_string(filename).expect("Something went wrong reading the file");
    let rows_list= contents.lines().collect::<Vec<&str>>();
    let rows = rows_list.len();
    let cols = rows_list[0].len();
    DMatrix::from_iterator(rows, cols, rows_list.concat().chars().map(|x| x.to_string()))
}

fn expand_matrix(matrix: GalaxyMatrix) -> GalaxyMatrix {
    let mut rows_to_insert = Vec::new();
    for (i, row) in matrix.row_iter().enumerate() {
        if row.iter().all(|x| x == ".") {
            rows_to_insert.push(i);
        }
    }
    let mut matrix = matrix;
    for row in rows_to_insert.iter().rev() {
       matrix = matrix.insert_rows(row + 1, 1, ".".to_owned());
    }
    let mut cols_to_insert = Vec::new();
    for (i, col) in matrix.column_iter().enumerate() {
        if col.iter().all(|x| x == ".") {
            cols_to_insert.push(i);
        }
    }
    for col in cols_to_insert.iter().rev() {
       matrix = matrix.insert_columns(col + 1, 1, ".".to_owned());
    }
    matrix
}

fn get_galaxies(matrix: GalaxyMatrix) -> Vec<(i32, i32)> {
    let mut galaxies = Vec::new();
    for (i, row) in matrix.row_iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell == "#" {
                galaxies.push((i as i32, j as i32));
            }
        }
    }
    galaxies
}

fn get_shortest_path(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn solution(filename: &str) -> i32 {
    let matrix = expand_matrix(get_matrix(filename));
    let galaxies = get_galaxies(matrix);
    let mut sum = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in galaxies.iter().skip(i + 1) {
            sum += get_shortest_path(*galaxy, *other_galaxy);
        }
    }
    sum
}

fn main() {
    assert_eq!(solution("example.txt"), 374);
    assert_eq!(solution("input.txt"), 9556896); 
    // assert_eq!(solution("example.txt", 10), 1030);
    // assert_eq!(solution("example.txt", 100), 8410);
    // assert_eq!(solution("input.txt", 1000000), 0); 
}
