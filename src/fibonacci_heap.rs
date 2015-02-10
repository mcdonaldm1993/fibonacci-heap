use std::collections::HashMap;
use std::collections::hash_map::Hasher;
use std::hash::Hash;
use std::collections::DList;
use std::num::Float;

use super::fibonacci_node::FibonacciNodeType;
use super::fibonacci_node::FibNode;
use super::fibonacci_node::remove_element;

/// Struct that represents the [Fibonacci Heap](http://en.wikipedia.org/wiki/Fibonacci_heap) data structure.
///
/// Algorithms for this are as seen in the [Introduction to Algorithms](http://en.wikipedia.org/wiki/Introduction_to_Algorithms) by Thomas H. Cormen, Charles E. Leiserson, Ronald L. Rivest, and Clifford Stein.
///
/// The key, K, is the priority used to order the heap. The value, V, is the data associated with the key.
pub struct FibonacciHeap<K, V> {
    hash_map: HashMap<V, FibonacciNodeType<K, V>>,
    roots: DList<FibonacciNodeType<K, V>>,
    min: Option<FibonacciNodeType<K, V>>,
    size: i32
}

impl<K, V> FibonacciHeap<K, V>
    where K: Clone + Eq + Ord,
          V: Clone + Eq + Hash<Hasher>
{
    /// Creates a new empty `FibonacciHeap`.
    pub fn new() -> FibonacciHeap<K, V> {
        FibonacciHeap{
            hash_map: HashMap::new(),
            roots: DList::new(),
            min: None,
            size: 0
        }
    }
    
    /// Inserts the value into the heap with priority key.
    pub fn insert(&mut self, key: K, value: V) -> () {
        let node: FibonacciNodeType<K, V> = FibNode::new(key, value.clone());
        self.hash_map.insert(value, node.clone());
        let min = self.min.clone();
        
        match min {
            Some(ref m) => {
                self.roots.push_front(node.clone());
                if node.get_key() < m.get_key() {
                    self.min = Some(node.clone());
                }
            },
            None => {
                self.roots = DList::new();
                self.roots.push_front(node.clone());
                self.min = Some(node.clone());
            }
        }
        
        self.size = self.size + 1;
    }
    
    /// Peeks at the minimum of the heap.
    ///
    /// Returns `None` if the heap is empty.
    pub fn minimum(&self) -> Option<(K, V)> {
        match self.min {
            Some(ref m) => Some((m.get_key().clone(), m.get_value().clone(),)),
            None => None 
        }
    }
    
    // pub fn union(&mut self, other: FibonacciHeap<K, V>) -> () {
    //   
    // }
    
    /// Exctracts the minimum of the heap.
    ///
    /// Returns `None` if the heap is empty.
    pub fn extract_min(&mut self) -> Option<(K, V)> {
        let z = self.min.clone();
        let mut result = None;
        
        match z {
            Some(z) => {
                let mut children = z.get_children();
                for child in children.iter_mut() {
                    child.set_parent(None);
                    self.roots.push_front(child.clone());
                }
                
                {
                    let roots = &mut self.roots;
                    remove_element(roots, z.clone());
                }
    
                {            
                    if self.roots.len() == 0 {
                        self.min = None;
                    } else {
                        let new_min = self.roots.front().unwrap().clone();
                        self.min = Some(new_min);
                        self.consolidate();
                    }
                }
                
                self.hash_map.remove(&z.get_value());
                self.size = self.size -1;
                result = Some(( z.get_key(), z.get_value() ));
            },
            None => { }
        }
        
        result
    }
    
    /// Decreases the priority of the value to the key.
    ///
    /// Returns `Err` if the value is not in the heap or if the key is greater than the current priority of the value.
    pub fn decrease_key(&mut self, value: V, key: K) -> Result<(), ()> {
        let x;
        
        {
            let hash_node = self.hash_map.get(&value);
            
            if hash_node.is_none() {
                return Err(()); 
            } else {
                x = hash_node.unwrap().clone();
            }
        }
        
        if key > x.get_key() {
            return Err(());
        }
        
        x.set_key(key);
        
        let y = x.get_parent();
        
        match y {
            Some(y_some) => {
                if x.get_key() < y_some.get_key() {
                    self.cut(x.clone(), y_some.clone());
                    self.cascading_cut(y_some.clone());
                }
            },
            None => { }
        }
        
        if x.get_key() < self.min.clone().unwrap().get_key() {
            self.min = Some(x);
        }
        
        Ok(())
    }
    
    // pub fn delete(&mut self, value: V) -> () {
    //    
    // }
    
    fn consolidate(&mut self) -> () {
        let base: f64 = (1.0 + 5.0.sqrt())/2.0;
        let log_n = (self.size as f64).log(base) as usize + 1;
        let mut array: Vec<Option<FibonacciNodeType<K, V>>> = (0..log_n).map(|_| None).collect();
        
        let roots = &mut self.roots.clone();
        for root in roots.iter() {
            let mut x = root.clone();
            let mut d = x.rank();
            loop {
                if array[d].clone().is_none() { break; }
                let mut y = array[d].clone().unwrap();
                if x.get_key() > y.get_key() {
                    let n = x.clone();
                    x = y.clone();
                    y = n;
                }
                self.heap_link(y.clone(), x.clone());
                array[d] = None;
                d = d + 1;
            }
            array[d] = Some(x.clone());
        }
        
        self.min = None;
        
        for i in 0..log_n {
            let min = self.min.clone();
            if array[i].clone().is_none() { continue; }
            
            if min.is_none() {
                self.roots = DList::new();
                self.roots.push_front(array[i].clone().unwrap());
                self.min = array[i].clone();
            } else {
                self.roots.push_front(array[i].clone().unwrap());
                if array[i].clone().unwrap().get_key() < min.unwrap().get_key() {
                    self.min = array[i].clone();
                }
            }
        }
    }
    
    fn heap_link(&mut self, y: FibonacciNodeType<K, V>, x: FibonacciNodeType<K, V>) -> () {
        let roots = &mut self.roots;
        remove_element(roots, y.clone());
        x.add_child(y.clone());
        y.set_marked(false);
    }
    
    fn cut(&mut self, x: FibonacciNodeType<K, V>, y: FibonacciNodeType<K, V>) -> () {
        y.remove_child(x.clone());
        self.roots.push_front(x.clone());
        x.set_parent(None);
        x.set_marked(true);
    }
    
    fn cascading_cut(&mut self, y: FibonacciNodeType<K, V>) -> () {
        let z = y.get_parent();
        
        match z {
            Some(z_some) => {
                if !y.is_marked() {
                    y.set_marked(true);
                } else {
                    self.cut(y, z_some.clone());
                    self.cascading_cut(z_some.clone())
                }
            },
            None => { }
        }
    }
}