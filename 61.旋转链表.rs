/*
 * @lc app=leetcode.cn id=61 lang=rust
 *
 * [61] 旋转链表
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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }

        let mut len = 1;
        let mut curr = head.as_ref();
        while let Some(node) = curr {
            if node.next.is_some() {
                len += 1;
                curr = node.next.as_ref();
            } else {
                break;
            }
        }

        let k = k as usize % len;
        if k == 0 {
            return head;
        }

        let mut new_tail = &mut head;
        for _ in 0..(len - k - 1) {
            new_tail = &mut new_tail.as_mut().unwrap().next;
        }

        let mut new_head = new_tail.as_mut().unwrap().next.take();
        
        let mut tail = new_head.as_mut().unwrap();
        while tail.next.is_some() {
            tail = tail.next.as_mut().unwrap();
        }
        
        tail.next = head;

        new_head
    }
}

// @lc code=end

