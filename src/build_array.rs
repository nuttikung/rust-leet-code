// println!("{:?}", build_array(vec![5,0,1,2,3,4]));

fn build_array(nums: Vec<i32>) -> Vec<i32> {
    let mut answer: Vec<i32> = Vec::new();
    for value_index in &nums {
        answer.push(nums[*value_index as usize]);
    }
    return answer;
}
