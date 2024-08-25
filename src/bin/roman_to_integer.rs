// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000

//For example, 2 is written as II in Roman numeral, just two ones added together. 
//12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.

// Roman numerals are usually written largest to smallest from left to right. 
//However, the numeral for four is not IIII. Instead, the number four is written as IV. 
//Because the one is before the five we subtract it making four. The same principle applies to the number nine, 
//which is written as IX. There are six instances where subtraction is used:

// I can be placed before V (5) and X (10) to make 4 and 9. 
// X can be placed before L (50) and C (100) to make 40 and 90. 
// C can be placed before D (500) and M (1000) to make 400 and 900.

use std::collections::HashMap;

struct Solution{
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman_int_map: HashMap<char, i32> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .iter()
        .cloned()
        .collect();

        let mut result: i32 = 0;
        let mut previous: i32 = 0;

        for c in s.chars().rev() {
            let current_number: i32 = roman_int_map[&c];
            if current_number < previous {
                result -= current_number;
            } else {
                result += current_number;
            }
            previous = current_number;
        }

        result
    }
}

fn main() {
    // **** Case1 ****
    //Parameters
    let s: String = String::from("III");
    //Expected
    let expected: i32 = 3;
    //Result
    let output: i32 = Solution::roman_to_int(s);
    //Compare 
    assert_eq!(output, expected);


    // **** Case2 ****
    //Parameters
    let s: String = String::from("LVIII");
    //Expected
    let expected: i32 = 58;
    //Result
    let output: i32 = Solution::roman_to_int(s);
    //Compare 
    assert_eq!(output, expected);


    // **** Case3 ****
    //Parameters
    let s: String = String::from("MCMXCIV");
    //Expected
    let expected: i32 = 1994;
    //Result
    let output: i32 = Solution::roman_to_int(s);
    //Compare 
    assert_eq!(output, expected);
}