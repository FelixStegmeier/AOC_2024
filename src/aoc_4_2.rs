use std::usize;

pub fn part_1(path: &std::path::Path) {
    let string = std::fs::read_to_string(path).unwrap();
    let matrix: Vec<Vec<char>> = string_to_2d_vec(string);
    scan_matrix_for_string(matrix, vec!['M', 'A', 'S']);
}

fn string_to_2d_vec(s: String) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = vec![];
    for row in s.split('\n') {
        let mut col: Vec<char> = vec![];
        for character in row.chars() {
            col.push(character);
        }
        matrix.push(col);
    }
    matrix
}

fn scan_matrix_for_string(matrix: Vec<Vec<char>>, search_for_string: Vec<char>) {
    let mut sum = 0;
    print!("\nrows: {}, cols: {}\n", matrix.len(), matrix[0].len());
    for row_index in 1..matrix.len()-1 {
        for col_index in 1..matrix[0].len() - 1 {
            if matrix[row_index][col_index] == search_for_string[1] {
                sum += search_cross_shape(
                    matrix.clone(),
                    row_index.clone(),
                    col_index.clone(),
                );
                print!("Summe: {}", sum);
            }
        }
    }
    print!("\n\nSumme: {}\n\n", sum);
}

fn search_cross_shape(
    matrix: Vec<Vec<char>>,
    row_index: usize,
    col_index: usize,
) -> i32 {
    let a =  matrix[row_index - 1][col_index - 1] == 'M' && matrix[row_index + 1][col_index + 1] == 'S';
    let b =  matrix[row_index + 1][col_index + 1] == 'M' && matrix[row_index - 1][col_index - 1] == 'S';
    let c =  matrix[row_index - 1][col_index + 1] == 'M' && matrix[row_index + 1][col_index - 1] == 'S';
    let d =  matrix[row_index + 1][col_index - 1] == 'M' && matrix[row_index - 1][col_index + 1] == 'S';
    if (a&&b)|| (a&&c)|| (a&&d) || (b&&c) || (b&&d)|| (c&&d){
        return 1;
    }
    return 0;
}
