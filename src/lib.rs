#![feature(core)]

use self::fibonacci_node::FibonacciNodeType;
pub use self::fibonacci_heap::FibonacciHeap;

mod fibonacci_heap;
mod fibonacci_node;