#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate fuzzsuite;
extern crate lockfree;

use fuzzsuite::*;
use lockfree::prelude::*;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
struct DequeMachine {
    deque: Arc<Deque<Box<u8>>>,
}

impl Spawn for DequeMachine {
    fn spawn() -> Self {
        Self::default()
    }

    fn fork(&self) -> Self {
        self.clone()
    }
}

impl Machine for DequeMachine {
    fn interpret(&mut self, mut byte: u8, bytecode: &mut Bytecode) {
        loop {
            let is_back = byte & 4 != 0;
            match byte % 4 {
                0 => {
                    byte = if is_back {
                        self.deque.pop_back()
                    } else {
                        self.deque.pop_front()
                    }
                    .map_or(1, |x| *x);
                },

                1 => {
                    if is_back {
                        self.deque.pop_back();
                    } else {
                        self.deque.pop_front();
                    }
                    break;
                },

                2 | 3 => {
                    let val = bytecode.next().unwrap_or(0);
                    if is_back {
                        self.deque.push_back(Box::new(val));
                    } else {
                        self.deque.push_front(Box::new(val));
                    }
                    break;
                },

                _ => unreachable!(),
            }
        }
    }
}

fuzz_target!(|data: &[u8]| {
    let _ = test::<DequeMachine>(Bytecode::no_symbols(data));
});
