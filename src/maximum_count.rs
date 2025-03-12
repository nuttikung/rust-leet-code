use std::cmp;

// println!("{:?}", maximum_count(vec![-2,-1,-1,1,2,3]))

fn maximum_count(nums: Vec<i32>) -> i32 {
    let mut negative_count: i32 = 0;
    let mut positive_count: i32 = 0;

    for value in nums.into_iter() {
        if value == 0 {
            continue;
        } else if value < 0 {
            negative_count+=1;
        } else {
            positive_count+=1;
        }
    }

    return cmp::max(negative_count, positive_count)

}
