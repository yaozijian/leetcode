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
    let n = Solution::reverse_k_group(h,3);
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
   
    pub fn reverse_k_group(head: Option<Box<ListNode>>,k : i32) -> Option<Box<ListNode>> {
            if head.is_none(){
                return None;
            }

            let k = k as usize;
            let mut v = vec![];

            let mut next = head;
            let mut n;
   
            while next.is_some(){
                n = next.as_mut().unwrap().next.take();
                v.push(next);
                if v.len() == k{
                    v[0].as_mut().unwrap().next = Solution::reverse_k_group(n,k as i32);
                    for i in 1..k{
                        v[i].as_mut().unwrap().next = v[i-1].take();
                    }
                    return v[k-1].take();
                }else{
                    next = n;
                }
            }

            let cnt = v.len();
            if cnt > 1{
                for i in (0..cnt-1).rev(){
                    v[i].as_mut().unwrap().next = v[i+1].take(); 
                }
            }

            v[0].take()
    }
}

