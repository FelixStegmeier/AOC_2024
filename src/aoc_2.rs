fn parser(path: &std::path::Path) -> Vec<Vec<i32>>{
    let s = std::fs::read_to_string(path).unwrap();
    let mut matrix: Vec<Vec<i32>> = vec!(); 
    for row in s.split('\n'){
        let mut buf: Vec<i32> = vec!();
        for (index, element) in  row.split(' ').enumerate(){
            buf.push(match element.trim().parse::<i32>(){
                Ok(ok) => {print!("{} ", ok); ok}
                Err(_e) => {print!("what"); continue},
            });
        }
        print!("\n");
        matrix.push(buf);
    }
    matrix
}

fn check_all_increasing_all_decreasing_by_max_3(row: Vec<i32>) -> bool{
    let mut increasing = false;
    let mut decreasing = false;
    let mut previous: i32 = row[0];
    for index in 1..row.len(){
        let diff =  previous - row[index];
        if diff.abs() <= 0 || diff.abs() > 3{
            return false;
        }
        if diff < 0{
            decreasing = true;
        }
        if diff > 0{
            increasing = true;
        }
        previous = row[index];
    }
    (increasing || decreasing) && !(increasing && decreasing)
}

fn brute_force(row: Vec<i32>) -> bool{
    for i in 0..row.len(){
        let mut row_clone = row.clone();
        row_clone.remove(i);
        if check_all_increasing_all_decreasing_by_max_3(row_clone) {
            return true;
        };
    }
    return false
}

pub fn entrypoint_1(path: &std::path::Path) -> i32{
    let matrix = parser(path);
    let mut count = 0;
    for row in matrix{
        if brute_force(row) {
            count += 1;
        }
    }
    print!("\n{}", count);
    count
}