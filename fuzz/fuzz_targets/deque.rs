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
            // TODO more operations.
            match byte % 4 {
                0 => byte = self.deque.pop_back().map_or(1, |x| *x),

                1 => {
                    self.deque.pop_back();
                    break;
                },

                2 | 3 => {
                    let val = bytecode.next().unwrap_or(0);
                    self.deque.push_back(Box::new(val));
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
