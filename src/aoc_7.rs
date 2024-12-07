pub fn äöü(path: &std::path::Path){
    let string = std::fs::read_to_string(path).unwrap();
    let mut matrix: Vec<Vec<i64>> = vec![];

    for line in string.split('\n'){
        let row: Vec<&str> = line.split(' ').collect();
        let mut row_int_arr: Vec<i64> = vec!();
        for number in row{
            row_int_arr.push(number.trim().replace(":", "").parse::<i64>()
            .unwrap());
        }
        matrix.push(row_int_arr);
    }

    let mut sum = 0;
    for row in matrix.clone(){
        sum += row_valid_2(row.clone());
    }

    println!("{}", sum);
}
fn row_valid_1(row: Vec<i64>) -> i64{
    let goal = row[0];
    let ceil: i64 = (2 as i64).pow((row.len() - 2) as u32);
    for i in 0..ceil{
        let mut curr_sum = row[1];
        for index in 2..row.len(){
            if (i >> index-2) & 1 == 1{
                curr_sum += row[index];
            }else {
                curr_sum *= row[index];
            }
        }
        if curr_sum == goal{
            return goal;
        }
    }
    0
}

fn row_valid_2(row: Vec<i64>) -> i64{
    let goal = row[0];
    let ceil: i64 = (3 as i64).pow((row.len() - 2) as u32);

    for i in 0..ceil{
        let mut curr_sum = row[1];
        for index in 2..row.len(){
            let digit = (i / (3 as i64).pow(index as u32 - 2)) % 3;
            if digit == 0{
                curr_sum += row[index];
            }else  if digit == 1{
                curr_sum *= row[index];
            }else {
                curr_sum = concat(curr_sum, row[index])
            }
        }
        if curr_sum == goal{
            return goal;
        }
    }
    0
}

fn concat(a: i64, b: i64) -> i64{
    (a.to_string() + &b.to_string()).parse::<i64>().unwrap()
}

fn print_arr(map_arr: Vec<Vec<i32>>) {
    for (row_index, row) in map_arr.clone().iter().enumerate() {
        for (col_index, element) in row.iter().enumerate() {
            print!(" {} ", element);
        }
        print!("\n");
    }
}


mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(row_valid_2(vec![7290, 6, 8, 6, 15]), 7290);
    }
}
