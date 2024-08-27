//Given a string s consisting of words and spaces, 
//return the length of the last word in the string.
//A word is a maximal substring consisting of non-space characters only.


struct Solution{
}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace()
            .last()
            .map(|word| word.len() as i32)
            .unwrap_or(0)
    }
}

fn main() {
    // **** Case1 ****
    //Parameters
    let s: String = String::from("Hello World");
    //Expected
    let expected: i32 = 5;
    //Result
    let output: i32 = Solution::length_of_last_word(s);
    //Compare 
    assert_eq!(output, expected);


    // **** Case2 ****
    //Parameters
    let s: String = String::from("   fly me   to   the moon  ");
    //Expected
    let expected: i32 = 4;
    //Result
    let output: i32 = Solution::length_of_last_word(s);
    //Compare 
    assert_eq!(output, expected);


    // **** Case3 ****
    //Parameters
    let s: String = String::from("luffy is still joyboy");
    //Expected
    let expected: i32 = 6;
    //Result
    let output: i32 = Solution::length_of_last_word(s);
    //Compare 
    assert_eq!(output, expected);
}