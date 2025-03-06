// println!("{:?}", find_missing_and_repeated_values(vec![vec![1,3], vec![2,2]]));
// println!("{:?}", find_missing_and_repeated_values(vec![vec![9,1,7],vec![8,9,2],vec![3,4,6]]));
use std::collections::HashSet;

fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let mut temp = HashSet::new();
    let len = grid.len() as i64 * grid.len() as i64;
    let sum: i64 = ((1 +  len) * len) / 2;
    let mut current: i64 = 0;

    // sum from grid value
    for index in &grid {
        for value in index {
            if !temp.contains(value) {
                temp.insert(value);
            }
            current+=*value as i64;
        }
    }

    // sum from set value
    let mut sum_temp: i64 = 0;
    for v in temp {
        sum_temp+=*v as i64;
    }

    return vec![(current-sum_temp).abs() as i32,(sum-sum_temp).abs()as i32]
}
