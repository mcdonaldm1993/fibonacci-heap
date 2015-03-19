use std::cell::RefCell;
use std::rc::Rc;
use std::collections::LinkedList;

pub type FibonacciNodeType<K, V> = Rc<RefCell<FibonacciNode<K, V>>>;

pub trait FibNode<K, V> {
    fn new(key: K, value: V) -> Self;
    fn get_key(&self) -> K;
    fn set_key(&self, key: K) -> ();
    fn get_value(&self) -> V;
    fn rank(&self) -> usize;
    fn is_marked(&self) -> bool;
    fn set_marked(&self, marked: bool) -> ();
    fn get_parent(&self) -> Option<Self>;
    fn set_parent(&self, parent: Option<Self>) -> ();
    fn add_child(&self, child: Self) -> ();
    fn remove_child(&self, child: Self) -> Option<Self>;
    fn get_children(&self) -> LinkedList<FibonacciNodeType<K, V>>;
}



pub struct FibonacciNode<K, V> {
    key: K,
    value: V,
    marked: bool,
    parent: Option<FibonacciNodeType<K, V>>,
    children: LinkedList<FibonacciNodeType<K, V>>
    // Rank is children.len()
}

impl<K, V> FibNode<K, V> for FibonacciNodeType<K, V>
    where K: Clone + Eq,
          V: Clone + Eq
{
    fn new(key: K, value: V) -> FibonacciNodeType<K, V> {
        Rc::new(RefCell::new(FibonacciNode {
            key: key,
            value: value,
            marked: false,
            parent: None,
            children: LinkedList::new()
        }))
    }
    
    fn get_key(&self) -> K {
        self.borrow().key.clone()
    }
    
    fn set_key(&self, key: K) -> () {
        self.borrow_mut().key = key;
    }
    
    fn get_value(&self) -> V {
        self.borrow().value.clone()
    }
    
    fn rank(&self) -> usize {
        self.borrow().children.len()
    }
    
    fn is_marked(&self) -> bool {
        self.borrow().marked
    }
    
    fn set_marked(&self, marked: bool) -> () {
        self.borrow_mut().marked = marked;
    }
    
    fn get_parent(&self) -> Option<FibonacciNodeType<K, V>> {
        self.borrow().parent.clone()
    }
    
    fn set_parent(&self, parent: Option<FibonacciNodeType<K, V>>) -> () {
        self.borrow_mut().parent = parent;
    }
    
    fn add_child(&self, child: FibonacciNodeType<K, V>) -> () {
        self.borrow_mut().children.push_back(child);
    }
    
    fn remove_child(&self, child: FibonacciNodeType<K, V>) -> Option<FibonacciNodeType<K, V>> {
        let children = &mut self.borrow_mut().children;
        
        remove_element(children, child)
    }
    
    fn get_children(&self) -> LinkedList<FibonacciNodeType<K, V>> {
        self.borrow().children.clone()
    }
}

impl<K, V> PartialEq for FibonacciNodeType<K, V>
    where K: Eq,
          V: Eq
{
    fn eq(&self, other: &FibonacciNodeType<K, V>) -> bool {
        self.borrow().value == other.borrow().value && self.borrow().key == other.borrow().key
    }
}

impl<K, V> Eq for FibonacciNodeType<K, V> where K: Eq, V: Eq { }

pub fn remove_element<T>(list: &mut LinkedList<T>, element: T) -> Option<T>
    where T: Eq
{
    for _ in 0..list.len() {
        if *list.front().unwrap() == element {
            return list.pop_front();
        }
        
        list.pop_back().map(|tail| {
            list.push_front(tail)
        });
    }
    
    None
}