// println!("{}", maximum_triplet_value(vec![12, 6, 1, 2, 7]));

fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let mut maximum: i64 = 0;
    for i in 0..nums.len() - 2 {
        for j in i + 1..nums.len() - 1 {
            for k in j + 1..nums.len() {
                let value: i64 = (nums[i] as i64 - nums[j] as i64) * nums[k] as i64;
                if value > maximum {
                    maximum = value;
                }
            }
        }
    }

    return maximum;
}
