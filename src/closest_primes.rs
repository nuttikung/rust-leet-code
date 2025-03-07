// println!("{:?}", closest_primes(10, 19));
// println!("{:?}", closest_primes(4, 6));

fn closest_primes(left: i32, right: i32) -> Vec<i32> {
    if left == 1 && right == 1 {
        vec![-1, -1];
    }

    let mut temp: Vec<i32> = Vec::new();

    for i in left..=right {
        if i == 1 {
            continue;
        }
        // using sieve mark
        let mut is_prime: bool = true;
        let mut j: i32 = 2;
        loop {
            if j * j > i {
                break;
            }

            if i % j == 0 {
                is_prime = false;
                break;
            }
            j += 1;
        }

        if is_prime {
            temp.push(i);
        }
    }

    if temp.len() < 2 {
        vec![-1, -1];
    }

    let mut diff: i32 = MAX;
    let mut ans: Vec<i32> = vec![-1, -1];
    let mut index: usize = 0;

    loop {
        if index+1 >= temp.len() {
            break;
        }

        if temp[index+1] - temp[index] < diff {
            diff = temp[index+1] - temp[index];
            ans = vec![temp[index], temp[index+1]];
        }

        index+=1;
    }

    ans
}
