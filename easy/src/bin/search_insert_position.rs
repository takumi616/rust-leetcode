//Given a sorted array of distinct integers and a target value, 
//return the index if the target is found. If not, 
//return the index where it would be if it were inserted in order.

//You must write an algorithm with O(log n) runtime complexity.

struct Solution{
}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = (nums.len() - 1) as i32;

        while left <= right {
            let middle: i32 = left + (right - left) / 2;
            if nums[middle as usize] == target {
                return middle
            } else if nums[middle as usize] < target {
                left = middle + 1;
            } else {
                right = middle - 1;
            }
        }

        left
    }
}

fn main() {
    // **** Case1 ****
    //Parameters
    let nums: Vec<i32> = vec![1,3,5,6];
    let target: i32 = 5;
    //Expected
    let expected: i32 = 2;
    //Result
    let output: i32 = Solution::search_insert(nums, target);
    //Compare 
    assert_eq!(output, expected);


    // **** Case2 ****
    //Parameters
    let nums: Vec<i32> = vec![1,3,5,6];
    let target: i32 = 2;
    //Expected
    let expected: i32 = 1;
    //Result
    let output: i32 = Solution::search_insert(nums, target);
    //Compare 
    assert_eq!(output, expected);


    // **** Case3 ****
    //Parameters
    let nums: Vec<i32> = vec![1,3,5,6];
    let target: i32 = 7;
    //Expected
    let expected: i32 = 4;
    //Result
    let output: i32 = Solution::search_insert(nums, target);
    //Compare 
    assert_eq!(output, expected);
}