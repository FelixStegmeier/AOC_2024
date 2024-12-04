use std::{io::Read, vec};

fn stuff(mut list_a: Vec<i32>, mut list_b: Vec<i32>) -> i32 {
    list_a.sort();
    list_b.sort();
    let mut sum: i32 = 0;
    for i in 0..list_a.len() {
        sum += (list_a[i] - list_b[i]).abs();
    }
    sum
}

fn input_to_list(mut file: std::fs::File) {
    let buffer: &mut [u8] = &mut [];
    file.read(buffer);
}

fn read_from_fs(path: &std::path::Path) -> [Vec<i32>; 2] {
    let s = std::fs::read_to_string(path).unwrap();
    let mut list_a: Vec<i32> = vec![];
    let mut list_b: Vec<i32> = vec![];

    for line in s.lines() {
        for (index, element) in line.split("   ").enumerate() {
            if (index == 0) {
                print!("{}   ", element);
                list_a.push(element.parse::<i32>().unwrap());
            } else {
                print!("{}\n", element);
                list_b.push(element.parse::<i32>().unwrap());
            }
        }
    }
    return [list_a, list_b];
}

pub fn entrypoint(path: &std::path::Path) {
    let s = read_from_fs(path);
    let i = stuff(s[0].clone(), s[1].clone());
    print!("result: {}", i);
}
