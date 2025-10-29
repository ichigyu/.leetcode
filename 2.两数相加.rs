/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = dummy.as_mut();

        let (mut p1, mut p2) = (l1.as_ref(), l2.as_ref());
        let mut carry = 0;
        while p1.is_some() || p2.is_some() || carry > 0 {
            let v1 = p1.map_or(0, |node| node.val);
            let v2 = p2.map_or(0, |node| node.val);

            let sum = (v1 + v2 + carry) % 10;
            carry = (v1 + v2 + carry) / 10;

            curr.next = Some(Box::new(ListNode::new(sum)));
            curr = curr.next.as_mut().unwrap();

            if let Some(node) = p1 {
                p1 = node.next.as_ref();
            }
            if let Some(node) = p2 {
                p2 = node.next.as_ref();
            }
        }

        dummy.next
    }
}
// @lc code=end

