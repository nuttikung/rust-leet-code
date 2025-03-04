// println!("{}", count_even(30))
fn count_even(num: i32) -> i32 {
    let mut count = 0;
    for i in 1..=num {
        let mut sum = 0;
        let mut temp = i;

        loop {
            if temp <= 9 {
                sum += temp;
                break;
            }
            sum += temp % 10;
            temp = temp / 10;
        }

        if sum % 2 == 0 {
            count += 1;
        }
    }

    return count;
}
