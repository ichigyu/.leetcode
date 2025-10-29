/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { next: head, val: 0});
        let (mut fast, mut slow) = (&dummy.clone(), dummy.as_mut());

        for _ in 0..n {
            if let Some(node) = &fast.next {
                fast = node;
            }
        }

        while let Some(node) = &fast.next {
            fast = fast.next.as_ref().unwrap();
            slow = slow.next.as_mut().unwrap();
        }

        slow.next = slow.next.as_mut().unwrap().next.take();

        dummy.next
    }
}
// @lc code=end

