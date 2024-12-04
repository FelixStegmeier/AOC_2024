use std::usize;

pub fn part_1(path: &std::path::Path) {
    let string = std::fs::read_to_string(path).unwrap();
    let matrix: Vec<Vec<char>> = string_to_2d_vec(string);
    scan_matrix_for_string(matrix, vec!['X', 'M', 'A', 'S']);
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
    for row_index in 0..matrix.len() {
        for col_index in 0..matrix[0].len() -1 {
            if matrix[row_index][col_index] == search_for_string[0] {
                sum += search_radially(
                    matrix.clone(),
                    row_index.clone(),
                    col_index.clone(),
                    search_for_string.clone(),
                );
                print!("Summe: {}", sum);
            }
        }
    }
    print!("\n\nSumme: {}\n\n", sum);
}

fn search_radially(
    matrix: Vec<Vec<char>>,
    row_index: usize,
    col_index: usize,
    search_for_string: Vec<char>,
) -> i32 {
    let mut right = true;
    let mut down = true;
    let mut left = true;
    let mut topright = true;
    let mut topleft = true;
    let mut botleft = true;
    let mut up = true;
    let mut botright = true;
    //scan_right row_index++
    for i in 1..search_for_string.len() {
        if row_index + search_for_string.len() > matrix.len() {
            right = false;
            break;
        }
        if !(matrix[row_index + i][col_index] == search_for_string[i]) {
            right = false;
            break;
        }
    }

    //scan_up col_index--
    for i in 1..search_for_string.len() {
        match col_index.checked_sub(search_for_string.len() - 1) {
            Some(s) => (),
            None => {
                up = false;
                break;
            }
        }
        if !(matrix[row_index][col_index - i] == search_for_string[i]) {
            up = false;
            break;
        }
    }
    //scan_left row_index--
    for i in 1..search_for_string.len() {
        match row_index.checked_sub(search_for_string.len() - 1) {
            Some(s) => (),
            None => {
                left = false;
                break;
            }
        }
        if !(matrix[row_index - i][col_index] == search_for_string[i]) {
            left = false;
            break;
        }
    }
    //scan_down col_index++
    for i in 1..search_for_string.len() {
        if col_index + search_for_string.len() > matrix[0].len() {
            down = false;
            break;
        }
        if !(matrix[row_index][col_index + i] == search_for_string[i]) {
            down = false;
            break;
        }
    }
    //scan top_left
    for i in 1..search_for_string.len() {
        match row_index.checked_sub(search_for_string.len() - 1) {
            Some(s) => (),
            None => {
                topleft = false;
                break;
            }
        }
        match col_index.checked_sub(search_for_string.len() - 1) {
            Some(s) => (),
            None => {
                topleft = false;
                break;
            }
        }
        if !(matrix[row_index - i][col_index - i] == search_for_string[i]) {
            topleft = false;
            break;
        }
    }
    //scan top_right
    for i in 1..search_for_string.len() {
        match col_index.checked_sub(search_for_string.len() - 1) {
            Some(s) => (),
            None => {
                topright = false;
                break;
            }
        }
        if row_index + search_for_string.len() > matrix.len() {
            topright = false;
            break;
        }
        if !(matrix[row_index + i][col_index - i] == search_for_string[i]) {
            topright = false;
            break;
        }
    }
    //scan bot_left
    for i in 1..search_for_string.len() {
        match row_index.checked_sub(search_for_string.len() - 1) {
            Some(s) => (),
            None => {
                botleft = false;
                break;
            }
        }
        if col_index + search_for_string.len() > matrix[0].len() {
            botleft = false;
            break;
        }
        if !(matrix[row_index - i][col_index + i] == search_for_string[i]) {
            botleft = false;
            break;
        }
    }
    //scan bot_right
    for i in 1..search_for_string.len() {
        if row_index + search_for_string.len() > matrix.len()
            || col_index + search_for_string.len() > matrix[0].len()
        {
            botright = false;
            break;
        }
        if !(matrix[row_index + i][col_index + i] == search_for_string[i]) {
            botright = false;
            break;
        }
    }
    let mut sum = 0;
    if up {
        sum += 1;
    }
    if right {
        sum += 1;
    }
    if down {
        sum += 1;
    }
    if left {
        sum += 1;
    }
    if topleft {
        sum += 1;
    }
    if topright {
        sum += 1;
    }
    if botleft {
        sum += 1;
    }
    if botright {
        sum += 1;
    }
    print!("\npartialSum: {}\n", sum);
    return sum;
}
