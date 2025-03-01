fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let length = nums.len()-1;
    let mut answer: Vec<i32> = Vec::new();

    let mut p1 = 0;
    loop {
        if p1 >= length  {
            break;
        }

        let mut p2 = p1+1;
        loop {
            if p2 > length {
                break;
            }

            if nums[p1] + nums[p2] == target {
                answer.push(p1.try_into().unwrap());
                answer.push(p2.try_into().unwrap());
                break;
            }

            p2+=1;
        }

        p1+=1;
    }

    return answer
}
