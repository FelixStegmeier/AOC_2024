use std::{thread::yield_now, vec};

pub fn dandadan(path: &std::path::Path) {
    let string = std::fs::read_to_string(path).unwrap();
    let mut input_matrix: Vec<Vec<char>> = vec![];
    let mut result_matrix: Vec<Vec<char>> = vec![];
    for line in string.split("\n").map(|r| r.trim()) {
        let mut row: Vec<char> = vec![];
        for c in line.chars() {
            row.push(c);
        }
        input_matrix.push(row);
    }
    result_matrix = vec![vec!['.'; input_matrix[0].len()]; input_matrix.len()];

    //print_arr(input_matrix);

    let matrix_of_antenna_groups: Vec<Vec<[usize; 2]>> = get_groups_of_antennae(input_matrix);

    //ueber die einzelnen Gruppen iterieren und in den gruppen jeder mit jedem
    for antenna_group in matrix_of_antenna_groups {
        for i in 0..antenna_group.len() {
            for j in i + 1..antenna_group.len() {
                let x_diff = antenna_group[i][0] as i32 - antenna_group[j][0] as i32;
                let y_diff = antenna_group[i][1] as i32 - antenna_group[j][1] as i32;

                //part 1
                // let mut a = [(antenna_group[i][0] as i32 + x_diff), (antenna_group[i][1] as i32 + y_diff)];
                // let mut b = [(antenna_group[j][0] as i32 - x_diff), (antenna_group[j][1] as i32 - y_diff)];
                // if !(a[0] < 0 || a[1] < 0 || a[0] >= result_matrix.len() as i32 || a[1] >= result_matrix[0].len() as i32){
                //     result_matrix[a[0] as usize][a[1]  as usize] = '#';

                // }

                // if !(b[0] < 0 || b[1] < 0 || b[0] >= result_matrix.len() as i32 || b[1] >= result_matrix[0].len() as i32){
                //     result_matrix[b[0]  as usize][b[1]  as usize] = '#';
                // }

                //part 2
                let mut a: [i32; 2] = [(antenna_group[i][0] as i32), (antenna_group[i][1] as i32)];
                let mut b = [(antenna_group[j][0] as i32), (antenna_group[j][1] as i32)];

                while !(a[0] < 0
                    || a[1] < 0
                    || a[0] >= result_matrix.len() as i32
                    || a[1] >= result_matrix[0].len() as i32)
                {
                    result_matrix[a[0] as usize][a[1] as usize] = '#';
                    a = [(a[0] + x_diff), (a[1] + y_diff)];
                }

                while !(b[0] < 0
                    || b[1] < 0
                    || b[0] >= result_matrix.len() as i32
                    || b[1] >= result_matrix[0].len() as i32)
                {
                    result_matrix[b[0] as usize][b[1] as usize] = '#';
                    b = [(b[0] - x_diff), (b[1] - y_diff)];
                }
            }
        }
    }

    let mut sum = 0;

    for row in result_matrix.clone() {
        for character in row {
            if character == '#' {
                sum += 1;
            }
        }
    }

    print_arr(result_matrix);
    println!("{}", sum)
}

fn get_groups_of_antennae(input_matrix: Vec<Vec<char>>) -> Vec<Vec<[usize; 2]>> {
    let mut matrix_of_antenna_groups: Vec<Vec<[usize; 2]>> = vec![];
    let mut antennae_characters: Vec<char> = vec![];
    for (input_row_index, input_row) in input_matrix.iter().enumerate() {
        for (input_col_index, input_character) in input_row.iter().enumerate() {
            if input_character == &'.' {
                continue;
            }
            let mut new_character = true;

            for (index, output_row) in matrix_of_antenna_groups.clone().iter().enumerate() {
                if *input_character == antennae_characters[index] {
                    //then append it here and break
                    new_character = false;
                    matrix_of_antenna_groups[index].push([input_row_index, input_col_index]);
                }
            }
            if new_character {
                antennae_characters.push(*input_character);
                matrix_of_antenna_groups.push(vec![[input_row_index, input_col_index]]);
            }
        }
    }

    matrix_of_antenna_groups
}

fn print_arr(map_arr: Vec<Vec<char>>) {
    for (row_index, row) in map_arr.clone().iter().enumerate() {
        for (col_index, element) in row.iter().enumerate() {
            print!("{}", element);
        }
        print!("\n");
    }
}
fn print_arr_1(map_arr: Vec<Vec<[usize; 2]>>) {
    for (row_index, row) in map_arr.clone().iter().enumerate() {
        for (col_index, element) in row.iter().enumerate() {
            print!("x: {}, y: {}", element[0], element[1]);
        }
        print!("\n");
    }
}
