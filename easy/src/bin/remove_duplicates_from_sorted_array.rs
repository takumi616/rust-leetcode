struct Solution{
}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut slow_pointer: usize = 1;

        for fast_pointer in 1..nums.len() {
            if nums[fast_pointer] != nums[fast_pointer-1] {
                nums[slow_pointer] = nums[fast_pointer];
                slow_pointer += 1
            }
        }

        slow_pointer as i32
    }
}

fn main() {
    // **** Case1 ****
    //Parameters
    let mut nums: Vec<i32> = vec![1,1,2];
    //Expected
    let expected: i32 = 2;
    //Result
    let output: i32 = Solution::remove_duplicates(&mut nums);
    //Compare 
    assert_eq!(output, expected);


    // **** Case2 ****
    //Parameters
    let mut nums: Vec<i32> = vec![0,0,1,1,1,2,2,3,3,4];
    //Expected
    let expected: i32 = 5;
    //Result
    let output: i32 = Solution::remove_duplicates(&mut nums);
    //Compare 
    assert_eq!(output, expected);
}