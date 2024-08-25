struct Solution{
}

impl Solution {
    pub fn is_palindrome(input: i32) -> bool {
        if input < 0 {
            return false;
        }

        let mut reversed: i32 = 0;
        let mut temp_input: i32 = input;
        while temp_input > 0 {
            reversed = (reversed * 10) + (temp_input % 10);
            temp_input /= 10;
        }

        return input == reversed;
    }
}

fn main() {
    // **** Case1 ****
    //Parameters
    let input: i32 = 121;
    //Expected
    let expected: bool = true;
    //Result
    let output: bool = Solution::is_palindrome(input);
    //Compare 
    assert_eq!(output, expected);


    // **** Case2 ****
    //Parameters
    let input: i32 = -121;
    //Expected
    let expected: bool = false;
    //Result
    let output: bool = Solution::is_palindrome(input);
    //Compare 
    assert_eq!(output, expected);


    // **** Case3 ****
    //Parameters
    let input: i32 = 10;
    //Expected
    let expected: bool = false;
    //Result
    let output: bool = Solution::is_palindrome(input);
    //Compare 
    assert_eq!(output, expected);

}