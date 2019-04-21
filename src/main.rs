fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq,Eq,Debug,Clone)]
struct ListNode{
    pub val : i32,
    pub next : Option<Box<ListNode>>,
}

impl ListNode{
    #[inline]
    fn new(v: i32) -> ListNode{
        ListNode{
            val: v,
            next: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        
        let mut len = 0;
        let mut cur = &head;
        while cur.is_some(){
            len += 1;
            cur = &cur.as_ref().unwrap().next;
        }

        k = k % len;

        if k == 0{
            return head;
        }
        
        let mut rest = len;
        let mut cur = &mut head;
        while cur.is_some() && rest != k{
            rest -= 1;
            cur = &cur.as_ref().unwrap().next;
        }


        None

    }
}
