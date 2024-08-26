//Write a function to find the longest common prefix string amongst an array of strings.
//If there is no common prefix, return an empty string "".

struct Solution{
} 

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let first_word: &str = &strs[0];
        let mut common_prefix: String = String::new();

        for i in 0..first_word.len() {
            let character: char = first_word.chars().nth(i).unwrap();
            for word in &strs[1..] {
                if let Some(value) = word.chars().nth(i) {
                    if value != character {
                        return common_prefix;
                    }
                } else {
                    return common_prefix;
                }
            }
            common_prefix.push(character);
        }

        common_prefix       
    }
}

fn main() {
    // **** Case1 ****
    //Parameters
    let strs = vec!["flower", "flow", "flight"].iter().map(|&s| s.to_string()).collect::<Vec<String>>();
    //Expected
    let expected: String = String::from("fl");
    //Result
    let output: String = Solution::longest_common_prefix(strs);
    //Compare 
    assert_eq!(output, expected);


    // **** Case2 ****
    //Parameters
    let strs: Vec<String> = vec!["dog", "racecar", "car"].iter().map(|&s| s.to_string()).collect::<Vec<String>>();
    //Expected
    let expected: String = String::new();
    //Result
    let output: String = Solution::longest_common_prefix(strs);
    //Compare 
    assert_eq!(output, expected);
}