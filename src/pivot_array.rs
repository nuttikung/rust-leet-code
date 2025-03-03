fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut index = 0;
    let mut left: Vec<i32> = Vec::new();
    let mut middle: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    loop {
        if index > nums.len() - 1 {
            break;
        }

        if nums[index] < pivot {
            left.push(nums[index]);
        } else if nums[index] > pivot {
            right.push(nums[index]);
        } else {
            middle.push(nums[index]);
        }

        index += 1;
    }

    left.extend(middle);
    left.extend(right);
    return left;
}
