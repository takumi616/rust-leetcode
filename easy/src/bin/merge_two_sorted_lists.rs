//You are given the heads of two sorted linked lists list1 and list2.
//Merge the two lists into one sorted list. The list should be made by 
//splicing together the nodes of the first two lists.
//Return the head of the merged linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

struct Solution {
}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(mut l1), Some(mut l2)) => {
                if l1.val < l2.val {
                    l1.next = Solution::merge_two_lists(l1.next, Some(l2));
                    Some(l1)
                } else {
                    l2.next = Solution::merge_two_lists(Some(l1), l2.next);
                    Some(l2)
                }
            }
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            (None, None) => None,
        }               
    }
}

fn main() {
    // **** Case1 ****
    //Parameters
    let list1: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None,
            })),
        })),
    }));
    let list2: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None,
            })),
        })),
    }));
    //Expected
    let expected: Vec<i32> = vec![1, 1, 2, 3, 4, 4];
    //Result
    let merged_list: Option<Box<ListNode>> = Solution::merge_two_lists(list1, list2);
    let mut output: Vec<i32> = vec![];
    let mut current_node = merged_list;
    while let Some(node) = current_node {
        output.push(node.val);
        current_node = node.next;
    }
    //Compare 
    assert_eq!(output, expected);



    // **** Case2 ****
    //Parameters
    let list1: Option<Box<ListNode>> = None;
    let list2: Option<Box<ListNode>> = None;
    //Expected
    let expected: Vec<i32> = vec![];
    //Result
    let merged_list: Option<Box<ListNode>> = Solution::merge_two_lists(list1, list2);
    let mut output: Vec<i32> = vec![];
    let mut current_node = merged_list;
    while let Some(node) = current_node {
        output.push(node.val);
        current_node = node.next;
    }
    //Compare 
    assert_eq!(output, expected);



    // **** Case3 ****
    //Parameters
    let list1: Option<Box<ListNode>> = None;
    let list2: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 0,
        next: None,
    }));
    //Expected
    let expected: Vec<i32> = vec![0];
    //Result
    let merged_list: Option<Box<ListNode>> = Solution::merge_two_lists(list1, list2);
    let mut output: Vec<i32> = vec![];
    let mut current_node = merged_list;
    while let Some(node) = current_node {
        output.push(node.val);
        current_node = node.next;
    }
    //Compare 
    assert_eq!(output, expected);
}