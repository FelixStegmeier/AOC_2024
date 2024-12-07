use std::vec;

pub fn aeiou(path: &std::path::Path) {
    let mut map_arr: Vec<Vec<char>> = vec![];
    let guard = "^v><";
    let string = std::fs::read_to_string(path).unwrap();

    for line in string.split("\n") {
        map_arr.push(line.trim().chars().collect::<Vec<char>>());
    }

    let mut pos: Vec<i32> = vec![];
    let mut dir: Vec<Vec<i32>> = vec![];
    let mut dir_index = 0;
    dir.push(vec![-1, 0]);
    dir.push(vec![0, 1]);
    dir.push(vec![1, 0]);
    dir.push(vec![0, -1]);

    let mut count_unique_positions = 0;
    for (row_index, row) in map_arr.clone().iter().enumerate() {
        for (col_index, element) in row.iter().enumerate() {
            if guard.contains(*element) {
                pos = vec![row_index.try_into().unwrap(), col_index.try_into().unwrap()];

                if element == &'^' {
                    dir_index = 0;
                } else if element == &'>' {
                    dir_index = 1;
                } else if element == &'v' {
                    dir_index = 2;
                } else {
                    dir_index = 3;
                }
                count_unique_positions += 1;

                map_arr[row_index][col_index] = 'X';
            }
            print!("{}", element);
        }
        print!("\n");
    }

    loop {
        println!("before: {}", count_unique_positions);
        print_arr(map_arr.clone());

        if match map_arr.get((pos[0] + dir[dir_index][0]) as usize) {
            Some(s) => match s.get((pos[1] + dir[dir_index][1]) as usize) {
                Some(character) => character == &'#',
                None => break,
            },
            None => break,
        } {
            //if map_arr[(pos[0] + dir[dir_index][0]) as usize][(pos[1] + dir[dir_index][1]) as usize] == '#'{
            dir_index = (dir_index + 1) % 4;
        }

        if map_arr[(pos[0] + dir[dir_index][0]) as usize][(pos[1] + dir[dir_index][1]) as usize]
            == 'X'
        {
        } else {
            map_arr[(pos[0] + dir[dir_index][0]) as usize][(pos[1] + dir[dir_index][1]) as usize] =
                'X';
            count_unique_positions += 1;
        }
        pos = vec![(pos[0] + dir[dir_index][0]), (pos[1] + dir[dir_index][1])];
        println!("after: {}", count_unique_positions);
    }
    println!("{}\n\n\n", count_unique_positions);
}

fn print_arr(map_arr: Vec<Vec<char>>) {
    for (row_index, row) in map_arr.clone().iter().enumerate() {
        for (col_index, element) in row.iter().enumerate() {
            print!("{}", element);
        }
        print!("\n");
    }
}
