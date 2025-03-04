// check_powers_of_three(12);
fn check_powers_of_three(n: i32) -> bool {
    let mut num = n;
    let mut trinary: Vec<i32> = Vec::new();

    loop {
        if num <= 0 {
            break;
        }

        trinary.push(num % 3);
        num = num / 3;
    }

    return !trinary.contains(&2);
}
