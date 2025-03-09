// println!("{}", number_of_alternating_groups(vec![0,1,0,1,0], 3));
// println!("{}", number_of_alternating_groups(vec![0,1,0,0,1,0,1], 6));
// println!("{}", number_of_alternating_groups(vec![1,1,0,1], 4));


fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
    let len = colors.len();
    let mut answer: i32 = 0;
    let mut left:usize = 0;
    let mut right:usize = 1;

    loop {
        if left > len-1 {
            break;
        }

        if colors[(right-1) % len] != colors[(right) % len] && (right-left) as i32 == k-1 {
            answer+=1;
            left+=1;
            right+=1;
        } else if colors[(right-1) % len] !=  colors[(right) % len] {
            right+=1;
        } else {
            left = right;
            right = left+1;
        }
    }
    answer
}
