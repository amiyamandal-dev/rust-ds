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

pub struct Solution {}

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.and_then(|mut n| -> Option<Box<ListNode>> {
            match n.next {
                Some(mut m) => {
                    n.next = Solution::swap_pairs(m.next);
                    m.next = Some(n);
                    Some(m)
                }
                None => Some(n),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_code() {}
}
