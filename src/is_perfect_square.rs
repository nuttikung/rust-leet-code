// println!("{}", is_perfect_square(16));
fn is_perfect_square(num: i32) -> bool {
    if num == 1 {
        return true
    }
    let mut i:i64= 2;
    let mut answer: bool = false;

    loop {
        if i*i == num as i64 {
            answer = true;
            break;
        }

        if i*i > num as i64 {
            break;
        }

        i+=1;
    }
    return answer;
}
