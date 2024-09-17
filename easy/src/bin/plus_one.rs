//You are given a large integer represented as an integer array digits, 
//where each digits[i] is the ith digit of the integer. 
//The digits are ordered from most significant to least significant in left-to-right order. 
//The large integer does not contain any leading 0's.

//Increment the large integer by one and return the resulting array of digits.

struct Solution{
}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        for i in (0..digits.len()).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            digits[i] = 0;
        }
        let mut result = vec![0; digits.len() + 1];
        result[0] = 1;
        result
    }
}

fn main() {
    // **** Case1 ****
    //Parameters
    let digits: Vec<i32> = vec![1, 2, 3];
    //Expected
    let expected: Vec<i32> = vec![1, 2, 4];
    //Result
    let output: Vec<i32> = Solution::plus_one(digits);
    //Compare 
    assert_eq!(output, expected);

    // **** Case2 ****
    //Parameters
    let digits: Vec<i32> = vec![4, 3, 2, 1];
    //Expected
    let expected: Vec<i32> = vec![4, 3, 2, 2];
    //Result
    let output: Vec<i32> = Solution::plus_one(digits);
    //Compare 
    assert_eq!(output, expected);

    // **** Case3 ****
    //Parameters
    let digits: Vec<i32> = vec![9];
    //Expected
    let expected: Vec<i32> = vec![1, 0];
    //Result
    let output: Vec<i32> = Solution::plus_one(digits);
    //Compare 
    assert_eq!(output, expected);
}