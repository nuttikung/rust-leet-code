println!("{}", three_consecutive_odds(vec![2, 6, 4, 1]));
println!(
    "{}",
    three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12])
);


fn is_odd(num: i32) -> bool {
    return num % 2 != 0;
}

fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    // negative case len (len less than 3)
    if arr.len() < 3 {
        return false;
    }

    let mut is_consecutive: bool = false;

    for i in 0..arr.len() - 2 {
        if is_odd(arr[i]) && is_odd(arr[i + 1]) && is_odd(arr[i + 2]) {
            is_consecutive = true;
            break;
        }
    }

    return is_consecutive;
}
