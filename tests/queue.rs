extern crate lockfree;

use lockfree::prelude::*;
use std::{
    sync::{atomic::AtomicUsize, Arc},
    thread,
};

#[test]
fn single_threaded_order() {
    let queue = Queue::new();
    assert_eq!(queue.pop(), None);
    assert_eq!(queue.pop(), None);
    queue.push(3);
    queue.push(5);
    assert_eq!(queue.pop(), Some(3));
    queue.push(7);
    queue.push(8);
    assert_eq!(queue.pop(), Some(5));
    assert_eq!(queue.pop(), Some(7));
    assert_eq!(queue.pop(), Some(8));
    assert_eq!(queue.pop(), None);
    assert_eq!(queue.pop(), None);
    queue.push(2);
    queue.push(0);
    assert_eq!(queue.pop(), Some(2));
    assert_eq!(queue.pop(), Some(0));
    assert_eq!(queue.pop(), None);
    assert_eq!(queue.pop(), None);
}

#[test]
fn multithreaded() {
    generic_multithreaded(20, 800, 55);
    generic_multithreaded(50, 10000, 87);
}

fn generic_multithreaded(nthread: usize, niter: usize, nmod: usize) {
    let queue = Arc::new(Queue::new());
    let mut handles = Vec::with_capacity(nthread);
    let _dbg = Arc::new(AtomicUsize::new(0));
    for i in 0..nthread {
        let queue = queue.clone();
        let _dbg = _dbg.clone();
        handles.push(thread::spawn(move || {
            for j in 0..niter {
                let val = (i * niter) + j;
                queue.push(val);
                if (val + 1) % nmod == 0 {
                    queue.pop();
                }
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let expected = niter * nthread - niter * nthread / nmod;
    let mut res = 0;
    while let Some(val) = queue.pop() {
        assert!(val < niter * nthread);
        res += 1;
    }
    assert_eq!(res, expected);
}
