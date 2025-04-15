// println!("{}", score_of_string(String::from("hello")));
// println!("{}", score_of_string(String::from("zaz")));

fn score_of_string(s: String) -> i32 {
    let s_clone = s.as_bytes();
    let str_len = s.len();
    let mut total: i32 = 0;

    for index in 0..str_len - 1 {
        let diff: i32;

        if s_clone[index] > s_clone[index + 1] {
            diff = (s_clone[index] - s_clone[index + 1]) as i32;
        } else {
            diff = (s_clone[index + 1] - s_clone[index]) as i32;
        }

        total += diff;
    }

    return total;
}
