use std::{i32, mem::swap};

pub fn asdf(path: &std::path::Path) {
    let string = std::fs::read_to_string(path).unwrap();
    let mut ruleset: Vec<String> = vec![];
    let mut updates: Vec<String> = vec![];
    for line in string.split("\n").to_owned() {
        if line.contains("|") {
            ruleset.push(line.to_string());
        } else if line.len() > 1 {
            updates.push(line.to_string());
        }
    }

    let mut sum = 0;
    let mut erroneous_updates: Vec<Vec<i32>> = vec![];

    let update_arr: Vec<Vec<i32>> = updates
        .iter()
        .map(|update_str| {
            update_str
                .trim()
                .split(',')
                .map(|str_num| str_num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    'outer: for update in update_arr {
        for rule in &ruleset {
            let split_rule = rule.split('|');
            let collected_num_str: Vec<&str> = split_rule.collect();
            if !check_rule(
                collected_num_str[0]
                    .trim()
                    .to_string()
                    .parse::<i32>()
                    .unwrap(),
                collected_num_str[1]
                    .trim()
                    .to_string()
                    .parse::<i32>()
                    .unwrap(),
                update.clone(),
            ) {
                erroneous_updates.push(update);
                continue 'outer;
            }
        }
    }

    for err_row in erroneous_updates {
        let mut sorted = false;
        let mut err_row = err_row.clone();

        while !sorted {
            sorted = true;
            for rule in &ruleset {
                let split_rule = rule.split('|');
                let collected_num_str: Vec<&str> = split_rule.collect();
                match check_rule_and_swap(
                    collected_num_str[0]
                        .trim()
                        .to_string()
                        .parse::<i32>()
                        .unwrap(),
                    collected_num_str[1]
                        .trim()
                        .to_string()
                        .parse::<i32>()
                        .unwrap(),
                    err_row.clone(),
                ) {
                    Some(swap_indices) => {
                        sorted = false;
                        let mut row = err_row.clone();
                        let tmp = row[swap_indices[0]];
                        row[swap_indices[0]] = row[swap_indices[1]];
                        row[swap_indices[1]] = tmp;
                        err_row = row;
                    }
                    None => (),
                };
            }
        }

        let mut num_arr: Vec<i32> = vec![];
        for num in err_row {
            num_arr.push(num);
        }
        sum += num_arr[num_arr.len() / 2];
    }

    print!("Summe: {}", sum);
}

fn check_rule_and_swap(num_1: i32, num_2: i32, update: Vec<i32>) -> Option<[usize; 2]> {
    let mut second_first = false;
    let mut first_index: usize = 0;
    let mut second_index: usize = 0;
    for (index, number) in update.iter().enumerate() {
        if *number == num_1 {
            first_index = index;
            if second_first {
                return Some([first_index, second_index]);
            }
            return None;
        }
        if *number == num_2 {
            second_index = index;
            second_first = true;
        }
    }
    None
}
fn check_rule(num_1: i32, num_2: i32, update: Vec<i32>) -> bool {
    let mut second_first = false;
    for number in update {
        if number == num_1 {
            if second_first {
                return false;
            }
            return true;
        }
        if number == num_2 {
            second_first = true;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(check_rule(97, 75, vec!(75, 979)), false)
    }
}
