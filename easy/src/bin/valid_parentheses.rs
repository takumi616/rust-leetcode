struct Solution{
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
        for value in s.chars() {
            if value == '(' || value == '{' || value == '[' {
                stack.push(value);
            } else {
                match (stack.pop(), value) {
                    (Some('('), ')') | (Some('{'), '}') | (Some('['), ']') => continue,
                    _ => return false,
                }
            }
        }
        stack.is_empty()
    }
}

fn main() {
    // **** Case1 ****
    //Parameters
    let input: String = String::from("()");
    //Expected
    let expected: bool = true;
    //Result
    let output: bool = Solution::is_valid(input);
    //Compare 
    assert_eq!(output, expected);


    // **** Case2 ****
    //Parameters
    let input: String = String::from("()[]{}");
    //Expected
    let expected: bool = true;
    //Result
    let output: bool = Solution::is_valid(input);
    //Compare 
    assert_eq!(output, expected);


    // **** Case3 ****
    //Parameters
    let input: String = String::from("(]");
    //Expected
    let expected: bool = false;
    //Result
    let output: bool = Solution::is_valid(input);
    //Compare 
    assert_eq!(output, expected);


    // **** Case4 ****
    //Parameters
    let input: String = String::from("([])");
    //Expected
    let expected: bool = true;
    //Result
    let output: bool = Solution::is_valid(input);
    //Compare 
    assert_eq!(output, expected);
}