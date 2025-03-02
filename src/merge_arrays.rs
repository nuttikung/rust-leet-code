// let input1 = vec![vec![2,4],vec![3,6],vec![5,5], vec![6,2]];
// let input2 = vec![vec![1,3],vec![4,3]];
// println!("{:?}", merge_arrays(input1,input2));
// println!("{:?}", merge_arrays(vec![vec![1,2], vec![2,3], vec![4,5]], vec![vec![1,4], vec![3,2], vec![4,1]]))
fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut answer: Vec<Vec<i32>>  = Vec::new();
    let mut p1 =0;
    let mut p2 =0;

    loop {
        if p1 > nums1.len()-1 || p2 > nums2.len()-1 {
            break;
        }

        if nums1[p1][0] < nums2[p2][0] {
            answer.push(vec![nums1[p1][0], nums1[p1][1]]);
            p1+=1;
        } else if  nums1[p1][0] > nums2[p2][0] {
            answer.push(vec![nums2[p2][0], nums2[p2][1]]);
            p2+=1;
        } else {
            answer.push(Vec::from([nums1[p1][0], nums1[p1][1] + nums2[p2][1]]));
            p1+=1;
            p2+=1;
        }
    }

    // check remain for nums1 or nums 2
    if p1 > nums1.len()-1 {
        for index in p2..nums2.len() {
            answer.push(vec![nums2[index][0], nums2[index][1]]);
        }
    }

    if p2 > nums2.len()-1 {
        for index in p1..nums1.len() {
            answer.push(vec![nums1[index][0], nums1[index][1]]);
        }
    }
    return answer;
}
