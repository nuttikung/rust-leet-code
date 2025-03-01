// let mut example: Vec<i32> = Vec::from([0,0,1,1,1,2,2,3,3,4]);
// println!("{}", remove_duplicates(&mut example))
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut p1 = 0;
    loop {
        if p1 >= nums.len()-1 {
            break;
        }
        let mut p2 = p1+1;
        loop {
            if p2 >= nums.len() {
                break;
            }

            if nums[p1] == nums[p2] {
                nums.remove(p2);
            } else {
                p2+=1;
            }
        }
        p1+=1;
    }
    return nums.len().try_into().unwrap();
}
