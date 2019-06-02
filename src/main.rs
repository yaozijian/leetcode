#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
 }
 
impl ListNode {
   #[inline]
   fn new(val: i32) -> Self {
     ListNode {
         next: None,
         val
     }
   }
 }

struct Solution;

fn main(){
    
    let add = |x,h|{
        let mut n = ListNode::new(x);
        n.next = h; 
        return Some(Box::new(n));
    };
    
    let mut h = None;
    
    h = add(5,h);
    h = add(4,h);
    h = add(3,h);
    h = add(2,h);
    h = add(1,h);

    Solution::show_list(&h);
    let n = Solution::swap_pairs(h);
    Solution::show_list(&n);
}


impl Solution {

    pub fn show_list(list : &Option<Box<ListNode>>){
        
        let mut cur = list;
        let mut node;
        
        while cur.is_some(){
            node = cur.as_ref().unwrap();
            print!("{} ->",node.val);
            cur = &node.next;
        }
        println!("");
    }
   
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		if head.is_none(){
            return None;
        }

        let mut head = head;
        let h = &mut head;
        let mut s = h.as_mut().unwrap().next.take();
        if s.is_none(){
            return head;

        }

        let t = s.as_mut().unwrap().next.take();
        h.as_mut().unwrap().next = Solution::swap_pairs(t);
        s.as_mut().unwrap().next = head; 
        s
    }
}

