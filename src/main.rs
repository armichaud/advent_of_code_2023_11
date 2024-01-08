use std::{fs::read_to_string, collections::BTreeSet};
use nalgebra::{DMatrix, Matrix, VecStorage, Dyn};

type GalaxyMatrix = Matrix<String, Dyn, Dyn, VecStorage<String, Dyn, Dyn>>;

fn get_matrix(filename: &str) -> GalaxyMatrix {
    let contents = read_to_string(filename).expect("Something went wrong reading the file");
    let rows_list= contents.lines().collect::<Vec<&str>>();
    let rows = rows_list.len();
    let cols = rows_list[0].len();
    DMatrix::from_iterator(rows, cols, rows_list.concat().chars().map(|x| x.to_string()))
}

fn expand_matrix(matrix: GalaxyMatrix, expand_factor: usize) -> GalaxyMatrix {
    let mut rows_to_insert = Vec::new();
    for (i, row) in matrix.row_iter().enumerate() {
        if row.iter().all(|x| x == ".") {
            rows_to_insert.push(i);
        }
    }
    let mut matrix = matrix;
    for row in rows_to_insert.iter().rev() {
       matrix = matrix.insert_rows(row + 1, expand_factor - 1, ".".to_owned());
    }
    let mut cols_to_insert = Vec::new();
    for (i, col) in matrix.column_iter().enumerate() {
        if col.iter().all(|x| x == ".") {
            cols_to_insert.push(i);
        }
    }
    for col in cols_to_insert.iter().rev() {
       matrix = matrix.insert_columns(col + 1, expand_factor - 1, ".".to_owned());
    }
    matrix
}

fn get_galaxies(matrix: GalaxyMatrix) -> Vec<(i64, i64)> {
    let mut galaxies = Vec::new();
    for (i, row) in matrix.row_iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell == "#" {
                galaxies.push((i as i64, j as i64));
            }
        }
    }
    galaxies
}

fn get_shortest_path(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn get_shortest_path_with_offsets(a: (i64, i64), b: (i64, i64), row_offsets: &Vec<i64>, col_offsets: &Vec<i64>) -> i64 {
    ((a.0 + row_offsets.get(a.0 as usize).unwrap()) - (b.0 + row_offsets.get(b.0 as usize).unwrap())).abs() + 
    ((a.1 + col_offsets.get(a.1 as usize).unwrap()) - (b.1 + col_offsets.get(b.1 as usize).unwrap())).abs()
}

fn solution(filename: &str, expand_factor: usize) -> i64 {
    let matrix = expand_matrix(get_matrix(filename), expand_factor);
    let galaxies = get_galaxies(matrix);
    let mut sum = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in galaxies.iter().skip(i + 1) {
            sum += get_shortest_path(*galaxy, *other_galaxy);
        }
    }
    sum
}

fn quick_solution(filename: &str, expand_factor: usize) -> i64 {
    let matrix = get_matrix(filename);
    let nrows = matrix.nrows() as i64;
    let ncols = matrix.ncols() as i64;
    let galaxies = get_galaxies(matrix);

    let mut row_set = BTreeSet::new();
    let mut col_set = BTreeSet::new();
    for i in 0..nrows {
        row_set.insert(i);
    }
    for i in 0..ncols {
        col_set.insert(i);
    }
    for galaxy in galaxies.iter() {
        row_set.remove(&galaxy.0);
        col_set.remove(&galaxy.1);
    }

    let mut row_offsets = Vec::new();
    let mut col_offsets = Vec::new();
    let mut offset = 0;
    for row in 0..nrows {
        row_offsets.push(offset);
        if row_set.contains(&row) {
            offset += expand_factor as i64 - 1;
        }
    }
    let mut offset = 0;
    for col in  0..ncols {
        col_offsets.push(offset);
        if col_set.contains(&col) {
            offset += expand_factor as i64 - 1;
        }
    }

    let mut sum = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in galaxies.iter().skip(i + 1) {
            sum += get_shortest_path_with_offsets(*galaxy, *other_galaxy, &row_offsets, &col_offsets);
        }
    }
    sum
}

fn main() {
    println!("{}", solution("example.txt", 2));
    println!("{}", solution("input.txt", 2)); 
    println!("{}", solution("example.txt", 10));
    println!("{}", solution("example.txt", 100));
    println!("{}", quick_solution("example.txt", 2));
    println!("{}", quick_solution("input.txt", 2)); 
    println!("{}", quick_solution("example.txt", 10));
    println!("{}", quick_solution("example.txt", 100));
    println!("{}", quick_solution("input.txt", 1000000)); 
}
