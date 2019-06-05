#[macro_use]
extern crate benchsuite;
extern crate lockfree;

use benchsuite::exec::Target;
use lockfree::deque::Deque;
use std::{
    collections::{LinkedList, VecDeque},
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Default)]
struct MutexVecTarget {
    inner: Arc<Mutex<VecDeque<u8>>>,
}

#[derive(Debug, Clone, Default)]
struct MutexListTarget {
    inner: Arc<Mutex<LinkedList<u8>>>,
}

#[derive(Debug, Clone, Default)]
struct LockfreeTarget {
    inner: Arc<Deque<u8>>,
}

impl Target for MutexVecTarget {
    #[inline(always)]
    fn round(&mut self) {
        let mut deque = self.inner.lock().unwrap();
        deque.pop_back();
        deque.push_front(234);
    }
}

impl Target for MutexListTarget {
    #[inline(always)]
    fn round(&mut self) {
        let mut deque = self.inner.lock().unwrap();
        deque.pop_back();
        deque.push_front(234);
    }
}

impl Target for LockfreeTarget {
    #[inline(always)]
    fn round(&mut self) {
        self.inner.pop_back();
        self.inner.push_front(234);
    }
}

fn main() {
    bench! {
        levels 1, 2, 4, 8, 16, 32;
        "mutex vector" => MutexVecTarget::default(),
        "mutex linked list" => MutexListTarget::default(),
        "lockfree" => LockfreeTarget::default(),
    }
}
