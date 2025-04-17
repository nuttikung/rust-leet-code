println!("{:?}", count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2))

fn is_divisible(value: i32, divisor: i32) -> bool {
    value % divisor == 0
}

fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
    let mut count: i32 = 0;

    for i in 0..nums.len() - 1 {
        for j in i+1..nums.len() {
            if nums[i] == nums[j] && is_divisible((i * j) as i32, k) {
                count += 1;
            }
        }
    }

    count
}
