// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

struct Solution;

// Runtime: 4 ms, faster than 100.00% of Rust online submissions for Add Two Numbers.
// Memory Usage: 2.3 MB, less than 100.00% of Rust online submissions for Add Two Numbers.

impl Solution {
  pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let (mut l1, mut l2) = (l1, l2);
    let mut l3 = Some(Box::new(ListNode::new(0)));
    let mut tail = &mut l3;
    let mut carry = 0;
    while l1.is_some() || l2.is_some() || carry == 1 {
      let x = l1.as_ref().map_or(0, |node| node.val);
      let y = l2.as_ref().map_or(0, |node| node.val);
      let mut sum = x + y + carry;
      carry = sum / 10;
      sum = sum % 10;
      tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
      tail = &mut tail.as_mut().unwrap().next;
      l1 = l1.and_then(|node| node.next);
      l2 = l2.and_then(|node| node.next);
    }
    l3.unwrap().next
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    let l1 = Some(Box::new(ListNode {
      val: 2,
      next: Some(Box::new(ListNode {
        val: 4,
        next: Some(Box::new(ListNode {
          val: 3,
          next: None,
        })),
      })),
    }));
    let l2 = Some(Box::new(ListNode {
      val: 5,
      next: Some(Box::new(ListNode {
        val: 6,
        next: Some(Box::new(ListNode {
          val: 4,
          next: None,
        })),
      })),
    }));
    assert_eq!(
      Some(Box::new(ListNode {
        val: 7,
        next: Some(Box::new(ListNode {
          val: 0,
          next: Some(Box::new(ListNode {
            val: 8,
            next: None,
          })),
        })),
      })),
      Solution::add_two_numbers(l1, l2)
    );
  }
}
