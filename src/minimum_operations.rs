use std::collections::HashMap;

// println!("{}", minimum_operations(vec![1, 2, 3, 4, 2, 3, 3, 5, 7]));
// println!("{}", minimum_operations(vec![4, 5, 6, 4, 4]));
// println!("{}", minimum_operations(vec![6, 7, 8, 9]));

// fn is_distinct(nums: HashMap<i32, i32>) -> bool {
//     let mut distinct: bool = true;
//     for (_, v) in nums {
//         if v > 1 {
//             distinct = false;
//             break;
//         }
//     }
//     return distinct;
// }

// fn minimum_operations(nums: Vec<i32>) -> i32 {
//     let mut copy_nums = nums.clone();
//     let mut ans: i32 = 0;
//     let mut frequency: HashMap<i32, i32> = HashMap::new();

//     for n in &copy_nums {
//         if frequency.contains_key(n) {
//             match frequency.get_mut(n) {
//                 Some(value) => {
//                     *value += 1;
//                 }
//                 None => {}
//             }
//         } else {
//             frequency.insert(*n, 1);
//         }
//     }

//     loop {
//         if is_distinct(frequency.clone()) {
//             break;
//         }

//         ans += 1;

//         let tmp_nums = copy_nums.clone();
//         let mut remain_len = 3;
//         if tmp_nums.len() < 3 {
//             remain_len = tmp_nums.len();
//         }

//         for index in 0..remain_len {
//             let element = copy_nums[index as usize];
//             match frequency.get_mut(&element) {
//                 Some(value) => {
//                     *value -= 1;
//                 }
//                 None => {}
//             }
//         }

//         while remain_len >= 1 {
//             copy_nums.remove(remain_len-1);
//             remain_len -=1;
//         }
//     }

//     return ans;
// }
