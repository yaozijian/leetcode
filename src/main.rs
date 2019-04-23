fn main() {
    let mut c = LRUCache::new(2);
    c.put(2,1);
    c.put(2,2);
    
    println!("{}",c.get(2));
    c.put(1,1);
    c.put(4,1);
    
    println!("{}",c.get(2));
}

use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Debug)]
struct LRUItem{
    val : i32,
    ver : u64,
}

struct LRUCache {
    max : usize,
    ver : u64,
    k2v : HashMap<i32,LRUItem>,
    v2v : BTreeMap<u64,i32>,
}

impl  LRUCache{

    fn new(capacity: i32) -> Self {
       LRUCache{
           max: capacity as usize,
           ver: 0,
           k2v: HashMap::with_capacity(capacity as usize),
           v2v: BTreeMap::new(),
       }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        let mut ret = -1;
        if let Some(ref mut item) = self.k2v.get_mut(&key){
            ret = item.val;
            self.v2v.remove(&item.ver);
            self.ver += 1;
            item.ver = self.ver;
            self.v2v.insert(item.ver,key);
        }
        ret
    }
    
    fn put(&mut self, key: i32, value: i32) {
        let mut kv : (u64,i32) = (0,0);
        if let Some(ref mut item) = self.k2v.get_mut(&key){
            self.v2v.remove(&item.ver);
            self.ver += 1;
            item.val = value;
            item.ver = self.ver;
            self.v2v.insert(item.ver,key);
            return;
        }else if self.k2v.len() >= self.max{
            let k = self.v2v.iter().nth(0).unwrap();
            kv.0 = *k.0;
            kv.1 = *k.1;
        }
        
        if kv.0 > 0{
            self.k2v.remove(&kv.1);
            self.v2v.remove(&kv.0);
        }
            
        self.ver += 1;
        let item = LRUItem{
            val: value,
            ver: self.ver,
        };
        self.k2v.insert(key,item);
        self.v2v.insert(self.ver,key);
    }
}

