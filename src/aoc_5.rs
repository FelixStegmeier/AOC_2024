use regex::Regex;
use std::{i32, num::TryFromIntError};

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
    'outer: for update in updates {
        for rule in &ruleset {
            let split_rule = rule.split('|');
            let collected_num_str: Vec<&str> = split_rule.collect();
            if !check_rule(
                collected_num_str[0].trim().to_string(),
                collected_num_str[1].trim().to_string(),
                update.trim().to_string(),
            ) {
                continue 'outer;
            }
        }
        let split_upd = update.split(',');
        let mut num_arr: Vec<i32> = vec![];
        for num in split_upd {
            num_arr.push(num.trim().parse::<i32>().unwrap());
        }
        sum += num_arr[num_arr.len() / 2];
    }
    print!("Summe: {}", sum);
}

fn check_rule(num_1: String, num_2: String, update: String) -> bool {
    let mut second_first = false;
    for number in update.split(',') {
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

    // let regex_check = format!("/({}).+({})/g", num_2.trim(), num_1);
    // let re = Regex::new(&regex_check).unwrap();

    // for _element in re.captures_iter(&update){
    //    return false;
    // }
    // true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            check_rule("97".to_string(), "75".to_string(), "75,97".to_string()),
            false
        )
    }
    #[test]

    fn test_02() {
        assert_eq!("61,13,29".split(',').collect::<Vec<&str>>()[2], "29")
    }
}
