use std::{process::id, vec};

pub fn aeiou(path: &std::path::Path) {
    let mut map: Vec<Vec<char>> = vec![];
    let map_of_logged_directions: Vec<Vec<usize>>;
    let guard = "^v><";
    let string: String = std::fs::read_to_string(path).unwrap();
    for line in string.split("\n") {
        map.push(line.trim().chars().collect::<Vec<char>>());
    }
    map_of_logged_directions = map.iter().map(|row| vec![100; row.len()]).collect();

    let mut starting_pos: Vec<i32> = vec![];
    let directions: Vec<Vec<i32>> = vec!(vec![-1, 0],vec![0, 1], vec![1, 0], vec![0, -1]);
    let mut dir_index: i32 = 0;

    for (row_index, row) in map.clone().iter().enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            if guard.contains(*cell) {
                starting_pos = vec![row_index.try_into().unwrap(), col_index.try_into().unwrap()];
                if cell == &'^' {
                    dir_index = 0;
                } else if cell == &'>' {
                    dir_index = 1;
                } else if cell == &'v' {
                    dir_index = 2;
                } else {
                    dir_index = 3;
                }
            }
            print!("{}", cell);
        }
        print!("\n");
    }

    let mut loop_count = 0;
    let starting_direction_index = dir_index.clone();
    // for (index_row, row) in map.clone().iter().enumerate() {
    //     for (index_col, cell) in row.iter().enumerate() {
    //         if cell == &'#' {
    //             continue;
    //         }
    //         if starting_pos[0] == index_row as i32 && starting_pos[1] == index_col as i32{
    //             continue;
    //         }

    //         let mut map_with_blockade: Vec<Vec<char>> = map.clone();
    //         map_with_blockade[index_row][index_col] = '#';

    //     }
    // }
    //loops through the positions the guard can get to and places the obstacle there
    let mut guard_current_position = starting_pos.clone();
    loop {
        if match map.get((guard_current_position[0] + directions[dir_index as usize][0]) as usize) {
            Some(s) => match s.get((guard_current_position[1] + directions[dir_index as usize][1]) as usize) {
                Some(character) => character == &'#',
                None => break,
            },
            None => break,
        } {
            //if map_arr[(pos[0] + dir[dir_index][0]) as usize][(pos[1] + dir[dir_index][1]) as usize] == '#'{
            dir_index = (dir_index + 1) % 4;
        }

        if map[(guard_current_position[0] + directions[dir_index as usize][0]) as usize][(guard_current_position[1] + directions[dir_index as usize][1]) as usize]
            == 'X'
        {
        } else {
            map[(guard_current_position[0] + directions[dir_index as usize][0]) as usize][(guard_current_position[1] + directions[dir_index as usize][1]) as usize] =
                'X';
        }
        guard_current_position = vec![(guard_current_position[0] + directions[dir_index as usize][0]), (guard_current_position[1] + directions[dir_index as usize][1])];
        if check_loop_for_position(starting_pos.clone(), directions.clone(), starting_direction_index, map.clone(), guard_current_position[0], guard_current_position[1], map_of_logged_directions.clone()){
            loop_count += 1;
        }
    }
    println!("{}\n\n\n", loop_count);
}

fn check_loop_for_position(starting_pos: Vec<i32>, directions: Vec<Vec<i32>>, starting_direction_index: i32, map: Vec<Vec<char>>, obstacle_index_row:i32, obstacle_index_col:i32, map_of_logged_directions: Vec<Vec<usize>>) -> bool{
    if starting_pos[0] == obstacle_index_row && starting_pos[1] == obstacle_index_col{
        false;
    }

    let mut map_with_blockade: Vec<Vec<char>> = map.clone();
    map_with_blockade[obstacle_index_row as usize][obstacle_index_col as usize] = '#';
    if is_loop(starting_pos.clone(), directions.clone(), starting_direction_index.clone(), map_with_blockade, map_of_logged_directions.clone()){
        return true;
    }
    false
}

fn is_loop(mut guard_original_pos: Vec<i32>, directions: Vec<Vec<i32>>, mut direction_index: i32, map: Vec<Vec<char>>, mut map_of_logged_directions: Vec<Vec<usize>>) -> bool{
    loop {
        if match map.get((guard_original_pos[0] + directions[direction_index as usize][0]) as usize) {
            Some(s) => match s.get((guard_original_pos[1] + directions[direction_index as usize][1]) as usize) {
                Some(character) => character == &'#',
                None => break,
            },
            None => break,
        } {
                direction_index = (direction_index + 1) % 4;
        }

        guard_original_pos = vec![(guard_original_pos[0] + directions[direction_index as usize][0]), (guard_original_pos[1] + directions[direction_index as usize][1])];

        if map_of_logged_directions[(guard_original_pos[0]) as usize][(guard_original_pos[1]) as usize]== direction_index as usize
        {
            return true;
        }
        
        map_of_logged_directions[(guard_original_pos[0]) as usize]
            [(guard_original_pos[1]) as usize] = direction_index as usize;
    }
    false
}



mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_s = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#.#^.....\n........#.\n#.........\n......#...";
        let mut map: Vec<Vec<char>> = vec![];
        for line in test_s.split("\n") {
            map.push(line.trim().chars().collect::<Vec<char>>());
        }
        let test_thingy: Vec<Vec<usize>> = vec![vec!(100;10); 10];
        assert_eq!(
            is_loop(vec!(6,4), vec!(vec![-1, 0],vec![0, 1], vec![1, 0], vec![0, -1]), 0, map, test_thingy),
            true
        )
    }
    #[test]
    fn test_2() {
        let test_s = "..#..#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        let mut map: Vec<Vec<char>> = vec![];
        for line in test_s.split("\n") {
            map.push(line.trim().chars().collect::<Vec<char>>());
        }
        let test_thingy: Vec<Vec<usize>> = vec![vec!(100;10); 10];
        assert_eq!(
            is_loop(vec!(6,4), vec!(vec![-1, 0],vec![0, 1], vec![1, 0], vec![0, -1]), 0, map, test_thingy),
            false
        )
    }
    #[test]
    fn test_check_loop_for_position() {
        let test_s = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#.#^.....\n........#.\n#.........\n......#...";
        let mut map: Vec<Vec<char>> = vec![];
        for line in test_s.split("\n") {
            map.push(line.trim().chars().collect::<Vec<char>>());
        }
        let test_thingy: Vec<Vec<usize>> = vec![vec!(100;10); 10];
        let map_of_logged_directions: Vec<Vec<usize>>= map.iter().map(|row| vec![100; row.len()]).collect();

        assert_eq!(check_loop_for_position(vec!(6,4), vec!(vec![-1, 0],vec![0, 1], vec![1, 0], vec![0, -1]), 0, map.clone(), 0, 0, map_of_logged_directions.clone()), true)
    }
}
