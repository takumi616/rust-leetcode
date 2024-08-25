use std::collections::HashMap;

struct Solution {
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_map: HashMap<i32, usize> = HashMap::new(); 
        for (i, v) in nums.into_iter().enumerate() {
            if let Some(index) = nums_map.get(&v) {
                return vec![*index as i32, i as i32];
            }
            nums_map.insert(target - v, i);
        }
    
        vec![]  
    }
}

fn main() {
    // **** Case1 ****
    //Parameters
    let nums: Vec<i32> = vec![2,7,11,15];
    let target: i32 = 9;
    //Expected
    let expected: Vec<i32> = vec![0, 1];
    //Result
    let output: Vec<i32> = Solution::two_sum(nums, target);
    //Compare 
    assert_eq!(output, expected);


    // **** Case2 ****
    //Parameters
    let nums: Vec<i32> = vec![3,2,4];
    let target: i32 = 6;
    //Expected
    let expected: Vec<i32> = vec![1, 2];
    //Result
    let output: Vec<i32> = Solution::two_sum(nums, target);
    //Compare 
    assert_eq!(output, expected);

    // **** Case3 ****
    //Parameters
    let nums: Vec<i32> = vec![3,3];
    let target: i32 = 6;
    //Expected
    let expected: Vec<i32> = vec![0, 1];
    //Result
    let output: Vec<i32> = Solution::two_sum(nums, target);
    //Compare 
    assert_eq!(output, expected);
}