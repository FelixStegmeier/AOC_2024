use std::vec;

pub fn dandadan(path: &std::path::Path){
    let string = std::fs::read_to_string(path).unwrap();
    let mut input_matrix: Vec<Vec<char>> = vec![];
    let mut result_matrix: Vec<Vec<char>> = vec![];
    for line in string.split("\n").map(|r|r.trim()){
        let mut row: Vec<char> = vec![];
        for c in line.chars(){
            row.push(c);
        }
        input_matrix.push(row);
    }
    result_matrix = vec![vec![' ' ;input_matrix[0].len()]; input_matrix.len()];

    print_arr(input_matrix);

    let matrix_of_antenna_groups: Vec<Vec<[i32; 2]>> = vec![];



    println!()
}

fn get_groups_of_antennae(input_matrix: Vec<Vec<char>>) -> Vec<Vec<[i32; 2]>>{
    let mut matrix_of_antenna_groups: Vec<Vec<[usize; 2]>> = vec![];
    let mut antennae_characters: Vec<char> = vec![];
    for (input_row_index, input_row) in input_matrix.iter().enumerate(){
        for (input_col_index, input_character) in input_row.iter().enumerate(){
            let mut new_character = true;

            for (index,output_row) in matrix_of_antenna_groups.clone().iter().enumerate(){

                if *input_character == antennae_characters[index]{
                    //then append it here and break
                    new_character = false;
                    matrix_of_antenna_groups[index].push([input_row_index, input_col_index]);
                }
                
            }
            if new_character{
                antennae_characters.push(*input_character);
                matrix_of_antenna_groups.push(vec![[input_row_index, input_col_index]]);
            }

        }
    }


    //ueber die einzelnen Gruppen iterieren und in den gruppen jeder mit jedem
    for antenna_group in matrix_of_antenna_groups{
        for i in 0..antenna_group.len(){
            for j in i+1..antenna_group.len(){
                //compare antenna_group[i] and antenna_group[j]
                let x_diff = antenna_group[i][0] - antenna_group[j][0];
                let y_diff = antenna_group[i][1] - antenna_group[j][1];
                //place 1 is antenna_group[i] - xy diff and 2 is antenna_group[j] + xy diff
                //possible i mixed x and y as well as +-

            }
        }

    }
    todo!()
}

fn print_arr(map_arr: Vec<Vec<char>>) {
    for (row_index, row) in map_arr.clone().iter().enumerate() {
        for (col_index, element) in row.iter().enumerate() {
            print!("{}", element);
        }
        print!("\n");
    }
}
