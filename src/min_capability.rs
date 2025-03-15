// println!("{}", min_capability(vec![2, 3, 5, 9], 2));
// println!("{}", min_capability(vec![2,7,9,3,1], 2));

fn is_possibly(nums: Vec<i32>, k: i32, finding: i32) -> bool {
    let mut is_possible: bool = false;
    let mut count = 0;
    // loop check value <= finding
    let mut index = 0;
    while index < nums.len() {
        if nums[index] <= finding {
            count += 1;
            index+=1;
        }

        if count >= k {
            is_possible = true;
            break;
        }
        index+=1;
    }
    // return possible steal
    return is_possible;
}

fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
    // region :      --- Finding Range
    let mut low = i32::MAX;
    let mut high = i32::MIN;
    for value in &nums {
        if *value < low {
            low = *value;
        }
        if *value > high {
            high = *value;
        }
    }
    // end region :  --- Finding Range

    // region :      --- Binary Search
    let mut ans: i32 = 0;

    while low <= high {
        let middle: i32 = low + (high - low) / 2;

        if is_possibly(nums.clone(), k, middle) {
            ans = middle;
            high = middle - 1;
        } else {
            low = middle + 1;
        }
    }

    // end region :  --- Binary Search

    return ans;
}
