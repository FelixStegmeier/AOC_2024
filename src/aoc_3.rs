use regex::Regex;

pub fn read(path: &std::path::Path){
    let s = std::fs::read_to_string(path).unwrap();
    let mut do_block = true;
    let mut res = 0;
    let regex_1 = r"mul\((\d+),(\d+)\)";
    let regex_2 = r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)";
    let re = Regex::new(&regex_2).unwrap();
    for captures in re.captures_iter(&s) {
        if captures.get(0).unwrap().as_str() == "don't()"{
            do_block = false;
            continue;
        }
        if captures.get(0).unwrap().as_str() == "do()"{
            do_block = true;
            continue;
        }
        if !do_block{
            continue;
        }
        print!("captures: {}\n", captures.get(0).unwrap().as_str());
        let first = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let second = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        print!("\nfirst: {}, second: {}", first, second);
        res += first*second;
    }
    print!("result: {}\n", res);
}