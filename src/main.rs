fn main() {
    apply_operations(Vec::from([1,2,2,1,1,0,523,523]));
}

fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
    let mut temp: Vec<i32>  = nums.to_vec();
    // Loop Apply Operation (Multiply).
    let mut index = 0;
    loop {
        if temp[index] == temp[index+1] && temp[index] != 0 {
            temp[index] = temp[index] * 2;
            temp[index+1] = 0;
        }

        index+=1;

        if index >= nums.len()-1 {
            break;
        }
    }

    // Loop shift zero.
    let zero_vec: Vec<i32> = temp.to_vec().into_iter().filter(|&value| value == 0).collect();
    let zero_count = zero_vec.len();
    let mut normal_vec: Vec<i32> = temp.to_vec().into_iter().filter(|&value| value != 0).collect();
    let mut fill_count = 1;

    loop {
        if fill_count > zero_count {
            break;
        }
        normal_vec.push(0);
        fill_count+=1;
    }
    return  normal_vec
}
