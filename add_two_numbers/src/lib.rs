#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::needless_pass_by_value)]
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    const fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}


#[must_use]
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    Some(Box::new(ListNode::new(0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3))),
            })),
        }));

        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));

        let result = add_two_numbers(l1, l2);

        assert_eq!(
            result,
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(8))),
                })),
            }))
        );
    }

    #[test]
    fn test_add_two_numbers_2() {
        let l1 = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode::new(1))),
        }));

        let l2 = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode::new(1))),
        }));

        let result = add_two_numbers(l1, l2);

        assert_eq!(
            result,
            Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(2))),
            }))
        );
    }

    #[test]
    fn test_add_two_numbers_3() {
        let l1 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode::new(9))),
            })),
        }));

        let l2 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode::new(9))),
        }));

        let result = add_two_numbers(l1, l2);

        assert_eq!(
            result,
            Some(Box::new(ListNode {
                val: 8,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode::new(0))),
                })),
            }))
        );
    }
}
