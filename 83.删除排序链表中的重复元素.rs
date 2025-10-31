/*
 * @lc app=leetcode.cn id=83 lang=rust
 *
 * [83] 删除排序链表中的重复元素
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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut curr = head.as_mut();

        while let Some(node_curr) = curr {
            while node_curr.next.is_some() && node_curr.next.as_ref().unwrap().val == node_curr.val {
                node_curr.next = node_curr.next.as_mut().unwrap().next.take();
            }
            curr = node_curr.next.as_mut();
        }

        head
    }
}
// @lc code=end

