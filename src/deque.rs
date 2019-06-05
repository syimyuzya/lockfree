use incin::Pause;
use owned_alloc::OwnedAlloc;
use ptr::bypass_null;
use std::{
    fmt,
    iter::FromIterator,
    mem::ManuallyDrop,
    ptr::{null_mut, NonNull},
    sync::atomic::{AtomicPtr, Ordering::*},
};

/// A lock-free general-purpose deque.
pub struct Deque<T> {
    anchor: AtomicPtr<Anchor<T>>,
    incin: SharedIncin<T>,
}

impl<T> Deque<T> {
    /// Creates a new empty deque.
    pub fn new() -> Self {
        // XXX check_null_align?
        Self::with_incin(SharedIncin::new())
    }

    /// Creates an empty deque using the passed shared incinerator.
    pub fn with_incin(incin: SharedIncin<T>) -> Self {
        let anchor = OwnedAlloc::new(Anchor::default()).into_raw().as_ptr();
        Self { anchor: AtomicPtr::new(anchor), incin }
    }

    /// Returns the shared incinerator used by this [`Deque`].
    pub fn incin(&self) -> SharedIncin<T> {
        self.incin.clone()
    }

    /// Pushes a new value to the back of the deque.
    pub fn push_back(&self, item: T) {
        let node = OwnedAlloc::new(Node::with_val(item));
        let node_ptr = node.raw().as_ptr();
        let mut new_anchor = OwnedAlloc::new(Anchor::<T>::default());

        let pause = self.incin.inner.pause();
        let mut anchor_ptr = self.anchor.load(Acquire);
        loop {
            // Safe because `anchor` is never null.
            let anchor_nnptr = unsafe { bypass_null(anchor_ptr) };
            let anchor_ref = unsafe { anchor_nnptr.as_ref() };
            if anchor_ref.back.is_null() {
                new_anchor.front = node_ptr;
                new_anchor.back = node_ptr;
                new_anchor.status = Status::Stable;
                // TODO DRY
                match self.anchor.compare_exchange_weak(
                    anchor_ptr,
                    new_anchor.raw().as_ptr(),
                    Release,
                    Relaxed,
                ) {
                    Ok(_) => {
                        node.into_raw();
                        new_anchor.into_raw();
                        pause.add_to_incin(Garbage::Anchor(unsafe {
                            OwnedAlloc::from_raw(anchor_nnptr)
                        }));
                        return;
                    },
                    Err(latest) => anchor_ptr = latest,
                }
            } else if anchor_ref.status == Status::Stable {
                node.prev.store(anchor_ref.back, Release); // TODO ordering
                new_anchor.front = anchor_ref.front;
                new_anchor.back = node_ptr;
                new_anchor.status = Status::PushingBack;
                match self.anchor.compare_exchange_weak(
                    anchor_ptr,
                    new_anchor.raw().as_ptr(),
                    Release,
                    Relaxed,
                ) {
                    Ok(_) => {
                        node.into_raw();
                        let new_anchor_ptr = new_anchor.into_raw().as_ptr();
                        pause.add_to_incin(Garbage::Anchor(unsafe {
                            OwnedAlloc::from_raw(anchor_nnptr)
                        }));
                        self.stablize(new_anchor_ptr, &pause);
                        return;
                    },
                    Err(latest) => anchor_ptr = latest,
                }
            } else {
                self.stablize(anchor_ptr, &pause);
                anchor_ptr = self.anchor.load(Acquire);
            }
        }
    }

    /// Pushes a new value to the front of the deque.
    pub fn push_front(&self, item: T) {
        // TODO DRY?
        let node = OwnedAlloc::new(Node::with_val(item));
        let node_ptr = node.raw().as_ptr();
        let mut new_anchor = OwnedAlloc::new(Anchor::<T>::default());

        let pause = self.incin.inner.pause();
        let mut anchor_ptr = self.anchor.load(Acquire);
        loop {
            // Safe because `anchor` is never null.
            let anchor_nnptr = unsafe { bypass_null(anchor_ptr) };
            let anchor_ref = unsafe { anchor_nnptr.as_ref() };
            if anchor_ref.front.is_null() {
                new_anchor.front = node_ptr;
                new_anchor.back = node_ptr;
                new_anchor.status = Status::Stable;
                // TODO DRY
                match self.anchor.compare_exchange_weak(
                    anchor_ptr,
                    new_anchor.raw().as_ptr(),
                    Release,
                    Relaxed,
                ) {
                    Ok(_) => {
                        node.into_raw();
                        new_anchor.into_raw();
                        pause.add_to_incin(Garbage::Anchor(unsafe {
                            OwnedAlloc::from_raw(anchor_nnptr)
                        }));
                        return;
                    },
                    Err(latest) => anchor_ptr = latest,
                }
            } else if anchor_ref.status == Status::Stable {
                node.next.store(anchor_ref.front, Release); // TODO ordering
                new_anchor.front = node_ptr;
                new_anchor.back = anchor_ref.back;
                new_anchor.status = Status::PushingFront;
                match self.anchor.compare_exchange_weak(
                    anchor_ptr,
                    new_anchor.raw().as_ptr(),
                    Release,
                    Relaxed,
                ) {
                    Ok(_) => {
                        node.into_raw();
                        let new_anchor_ptr = new_anchor.into_raw().as_ptr();
                        pause.add_to_incin(Garbage::Anchor(unsafe {
                            OwnedAlloc::from_raw(anchor_nnptr)
                        }));
                        self.stablize(new_anchor_ptr, &pause);
                        return;
                    },
                    Err(latest) => anchor_ptr = latest,
                }
            } else {
                self.stablize(anchor_ptr, &pause);
                anchor_ptr = self.anchor.load(Acquire);
            }
        }
    }

    /// Pops a single element from the back of the deque.
    pub fn pop_back(&self) -> Option<T> {
        let pause = self.incin.inner.pause();
        let mut anchor_ptr = self.anchor.load(Acquire);
        let mut new_anchor = OwnedAlloc::new(Anchor::default());
        loop {
            // Safe because anchor is never null.
            let anchor_nnptr = unsafe { bypass_null(anchor_ptr) };
            let anchor_ref = unsafe { anchor_nnptr.as_ref() };
            if anchor_ref.back.is_null() {
                return None;
            }
            if anchor_ref.front == anchor_ref.back {
                new_anchor.front = null_mut();
                new_anchor.back = null_mut();
                new_anchor.status = Status::Stable;
                match self.anchor.compare_exchange_weak(
                    anchor_ptr,
                    new_anchor.raw().as_ptr(),
                    Release,
                    Relaxed,
                ) {
                    Ok(_) => break,
                    Err(latest) => anchor_ptr = latest,
                }
            } else if anchor_ref.status == Status::Stable {
                // Safe because the incinerator is paused.
                let back_ref = unsafe { &*anchor_ref.back };
                let prev = back_ref.prev.load(Acquire);
                new_anchor.front = anchor_ref.front;
                new_anchor.back = prev;
                new_anchor.status = Status::PoppingBack;
                match self.anchor.compare_exchange_weak(
                    anchor_ptr,
                    new_anchor.raw().as_ptr(),
                    Release,
                    Relaxed,
                ) {
                    Ok(_) => break,
                    Err(latest) => anchor_ptr = latest,
                }
            } else {
                self.stablize(anchor_ptr, &pause);
                anchor_ptr = self.anchor.load(Acquire);
            }
        }
        let anchor_nnptr = unsafe { bypass_null(anchor_ptr) };
        let anchor_ref = unsafe { anchor_nnptr.as_ref() };
        new_anchor.into_raw();
        let mut back_nnptr = unsafe { bypass_null(anchor_ref.back) };
        let val = unsafe { (&mut *back_nnptr.as_mut().val as *mut T).read() };
        pause.add_to_incin(Garbage::Anchor(unsafe {
            OwnedAlloc::from_raw(anchor_nnptr)
        }));
        pause.add_to_incin(Garbage::Node(unsafe {
            OwnedAlloc::from_raw(back_nnptr)
        }));
        return Some(val);
    }

    /// Pops a single element from the back of the deque.
    pub fn pop_front(&self) -> Option<T> {
        let pause = self.incin.inner.pause();
        let mut anchor_ptr = self.anchor.load(Acquire);
        let mut new_anchor = OwnedAlloc::new(Anchor::default());
        loop {
            // Safe because anchor is never null.
            let anchor_nnptr = unsafe { bypass_null(anchor_ptr) };
            let anchor_ref = unsafe { anchor_nnptr.as_ref() };
            if anchor_ref.front.is_null() {
                return None;
            }
            if anchor_ref.front == anchor_ref.back {
                new_anchor.front = null_mut();
                new_anchor.back = null_mut();
                new_anchor.status = Status::Stable;
                match self.anchor.compare_exchange_weak(
                    anchor_ptr,
                    new_anchor.raw().as_ptr(),
                    Release,
                    Relaxed,
                ) {
                    Ok(_) => break,
                    Err(latest) => anchor_ptr = latest,
                }
            } else if anchor_ref.status == Status::Stable {
                // Safe because the incinerator is paused.
                let front_ref = unsafe { &*anchor_ref.front };
                let next = front_ref.next.load(Acquire);
                new_anchor.front = next;
                new_anchor.back = anchor_ref.back;
                new_anchor.status = Status::PoppingFront;
                match self.anchor.compare_exchange_weak(
                    anchor_ptr,
                    new_anchor.raw().as_ptr(),
                    Release,
                    Relaxed,
                ) {
                    Ok(_) => break,
                    Err(latest) => anchor_ptr = latest,
                }
            } else {
                self.stablize(anchor_ptr, &pause);
                anchor_ptr = self.anchor.load(Acquire);
            }
        }
        let anchor_nnptr = unsafe { bypass_null(anchor_ptr) };
        let anchor_ref = unsafe { anchor_nnptr.as_ref() };
        new_anchor.into_raw();
        let mut front_nnptr = unsafe { bypass_null(anchor_ref.front) };
        let val = unsafe { (&mut *front_nnptr.as_mut().val as *mut T).read() };
        pause.add_to_incin(Garbage::Anchor(unsafe {
            OwnedAlloc::from_raw(anchor_nnptr)
        }));
        pause.add_to_incin(Garbage::Node(unsafe {
            OwnedAlloc::from_raw(front_nnptr)
        }));
        return Some(val);
    }

    /// Tries to fix node links.
    fn stablize(&self, anchor: *mut Anchor<T>, pause: &Pause<Garbage<T>>) {
        // Safe because the incinerator is still paused.
        let anchor_nnptr = unsafe { bypass_null(anchor) };
        let anchor_ref = unsafe { anchor_nnptr.as_ref() };
        match anchor_ref.status {
            Status::PoppingBack => {
                // Reset the pointer at the front/back to prevent ABA.
                let back_ref = unsafe { &*anchor_ref.back };
                let back_next = back_ref.next.load(Acquire);
                if self.anchor.load(Acquire) != anchor {
                    return;
                }
                if !back_next.is_null() {
                    if back_ref
                        .next
                        .compare_exchange_weak(
                            back_next,
                            null_mut(),
                            Release,
                            Relaxed,
                        )
                        .is_err()
                    {
                        return;
                    }
                }
            },
            Status::PoppingFront => {
                // TODO DRY?
                // Reset the pointer at the front/back to prevent ABA.
                let front_ref = unsafe { &*anchor_ref.front };
                let front_prev = front_ref.prev.load(Acquire);
                if self.anchor.load(Acquire) != anchor {
                    return;
                }
                if !front_prev.is_null() {
                    if front_ref
                        .prev
                        .compare_exchange_weak(
                            front_prev,
                            null_mut(),
                            Release,
                            Relaxed,
                        )
                        .is_err()
                    {
                        return;
                    }
                }
            },
            Status::PushingBack => {
                let prev = unsafe { (&*anchor_ref.back).prev.load(SeqCst) };
                let prev_next = unsafe { (&*prev).next.load(Acquire) };
                if self.anchor.load(Acquire) != anchor {
                    return;
                }
                if prev_next != anchor_ref.back {
                    let prev_ref = unsafe { &*prev };
                    if prev_ref
                        .next
                        .compare_exchange_weak(
                            prev_next,
                            anchor_ref.back,
                            Release,
                            Relaxed,
                        )
                        .is_err()
                    {
                        return;
                    }
                }
            },
            Status::PushingFront => {
                // TODO DRY?
                let next = unsafe { (&*anchor_ref.front).next.load(SeqCst) };
                let next_prev = unsafe { (&*next).prev.load(Acquire) };
                if self.anchor.load(Acquire) != anchor {
                    return;
                }
                if next_prev != anchor_ref.front {
                    let next_ref = unsafe { &*next };
                    if next_ref
                        .prev
                        .compare_exchange_weak(
                            next_prev,
                            anchor_ref.front,
                            Release,
                            Relaxed,
                        )
                        .is_err()
                    {
                        return;
                    }
                }
            },
            Status::Stable => {},
        }
        // On success, try to mark the state as Stable.
        let new_anchor = OwnedAlloc::new(Anchor::new(
            anchor_ref.front,
            anchor_ref.back,
            Status::Stable,
        ));
        if self
            .anchor
            .compare_exchange_weak(
                anchor,
                new_anchor.raw().as_ptr(),
                Release,
                Relaxed,
            )
            .is_ok()
        {
            new_anchor.into_raw();
            pause.add_to_incin(Garbage::Anchor(unsafe {
                OwnedAlloc::from_raw(anchor_nnptr)
            }));
        }
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {}
    }
}

impl<T> fmt::Debug for Deque<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "Deque {} anchor: {:?}, incin: {:?} {}",
            '{', self.anchor, self.incin, '}'
        )
    }
}

unsafe impl<T> Send for Deque<T> where T: Send {}
unsafe impl<T> Sync for Deque<T> where T: Send {}

#[derive(Debug)]
struct Anchor<T> {
    front: *mut Node<T>,
    back: *mut Node<T>,
    status: Status,
}

impl<T> Anchor<T> {
    fn new(front: *mut Node<T>, back: *mut Node<T>, status: Status) -> Self {
        Self { front, back, status }
    }
}

impl<T> Default for Anchor<T> {
    fn default() -> Self {
        Self::new(null_mut(), null_mut(), Status::Stable)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Stable,
    PushingFront,
    PushingBack,
    PoppingFront,
    PoppingBack,
}

#[derive(Debug)]
struct Node<T> {
    val: ManuallyDrop<T>,
    prev: AtomicPtr<Node<T>>,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new(val: T, prev: *mut Node<T>, next: *mut Node<T>) -> Self {
        Self {
            val: ManuallyDrop::new(val),
            prev: AtomicPtr::new(prev),
            next: AtomicPtr::new(next),
        }
    }

    fn with_val(val: T) -> Self {
        Self::new(val, null_mut(), null_mut())
    }
}

make_shared_incin! {
    { "[`Deque`]" }
    pub SharedIncin<T> of Garbage<T>
}

impl<T> fmt::Debug for SharedIncin<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "SharedIncin {} inner: {:?} {}", '{', self.inner, '}')
    }
}

enum Garbage<T> {
    Node(OwnedAlloc<Node<T>>),
    Anchor(OwnedAlloc<Anchor<T>>),
}

impl<T> fmt::Debug for Garbage<T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Garbage::Node(ptr) => write!(fmtr, "Garbage::Node({:?})", ptr),
            Garbage::Anchor(ptr) => write!(fmtr, "Garbage::Anchor({:?})", ptr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{sync::Arc, thread};

    #[test]
    fn simple() {
        let deque: Deque<i32> = Deque::new();

        assert_eq!(None, deque.pop_back());

        deque.push_back(42);
        assert_eq!(Some(42), deque.pop_back());
        assert_eq!(None, deque.pop_back());

        deque.push_back(43);
        deque.push_back(44);
        assert_eq!(Some(44), deque.pop_back());
        assert_eq!(Some(43), deque.pop_back());
        assert_eq!(None, deque.pop_back());
    }

    #[test]
    fn push_pop() {
        let deque: Deque<i32> = Deque::new();
        deque.push_back(45);

        deque.push_back(46);
        assert_eq!(Some(46), deque.pop_back());

        println!("pushing 47");
        deque.push_back(47);
        println!("popping 47");
        assert_eq!(Some(47), deque.pop_back());
        println!("popping 45");
        assert_eq!(Some(45), deque.pop_back());
        println!("popping (empty)");
        assert_eq!(None, deque.pop_back());
    }

    // TODO DRY?
    #[test]
    fn concurrent_push() {
        let deque = Arc::new(Deque::<i32>::new());
        let num_threads = 4;
        let num_vals = 32;
        let handles: Vec<_> = (0 .. num_threads)
            .map(|k| {
                let dq = Arc::clone(&deque);
                thread::spawn(move || {
                    for i in 0 .. num_vals {
                        let x = k * num_vals + i;
                        // println!("[T{}] Pushing: {}", k, x);
                        dq.push_back(x);
                        // println!("  [T{}] Pushed: {}", k, x);
                    }
                })
            })
            .collect();
        for handle in handles {
            handle.join().unwrap();
        }
        let mut vals = Vec::new();
        while let Some(v) = deque.pop_back() {
            vals.push(v);
        }
        println!("Values: {:?}", vals);
        vals.sort_unstable();
        let expected = Vec::from_iter(0 .. (num_threads * num_vals));
        assert_eq!(expected, vals);
    }

    #[test]
    fn concurrent_pop() {
        let deque = Arc::new(Deque::<i32>::new());
        let num_threads = 4;
        let num_vals = 32;
        for i in 0 .. (num_threads * num_vals) {
            deque.push_back(i);
        }
        let handles: Vec<_> = (0 .. num_threads)
            .map(|k| {
                let dq = Arc::clone(&deque);
                thread::spawn(move || {
                    let mut vals = Vec::new();
                    for _ in 0 .. num_vals {
                        // println!("[T{}] Popping", k);
                        let res = dq.pop_back();
                        match res {
                            Some(v) => {
                                // println!("  [T{}] Got: {}", k, v);
                                vals.push(v);
                            },
                            None => panic!("[T{}] Failed", k),
                        }
                    }
                    vals
                })
            })
            .collect();
        let mut all_vals = Vec::new();
        for (i, handle) in handles.into_iter().enumerate() {
            let vec = handle.join().unwrap();
            println!("Values (T{}): {:?}", i, vec);
            all_vals.extend(vec);
        }
        all_vals.sort_unstable();
        let expected = Vec::from_iter(0 .. (num_threads * num_vals));
        assert_eq!(expected, all_vals);
    }

    #[test]
    fn concurrent_push_pop_back() {
        let deque = Arc::new(Deque::<i32>::new());
        let num_threads = 20;
        let num_vals = 800;
        let modulo = 55;
        let handles: Vec<_> = (0 .. num_threads)
            .map(|k| {
                let dq = Arc::clone(&deque);
                thread::spawn(move || {
                    let mut vals = Vec::new();
                    for i in 0 .. num_vals {
                        let x = num_vals * k + i;
                        if (x + 1) % modulo == 0 {
                            // println!("[T{}] Popping", k);
                            match dq.pop_back() {
                                Some(v) => {
                                    // println!("  [T{}] Got: {}", k, v);
                                    vals.push(v);
                                },
                                None => panic!("[T{}] Failed", k),
                            }
                        } else {
                            // println!("[T{}] Pushing: {}", k, x);
                            dq.push_back(x);
                            // println!("  [T{}] Pushed", k);
                        }
                    }
                    vals
                })
            })
            .collect();
        let vecs: Vec<_> =
            handles.into_iter().map(|h| h.join().unwrap()).collect();
        let mut all_vals = Vec::new();
        for (i, vals) in vecs.into_iter().enumerate() {
            println!("Values (T{}): {:?}", i, vals);
            all_vals.extend(vals)
        }
        let mut dq_vals = Vec::new();
        while let Some(v) = deque.pop_back() {
            dq_vals.push(v);
        }
        println!("Values (deque): {:?}", dq_vals);
        all_vals.extend(dq_vals);
        all_vals.sort_unstable();
        let expected: Vec<_> = (0 .. (num_threads * num_vals))
            .filter(|i| (i + 1) % modulo != 0)
            .collect();
        assert_eq!(expected, all_vals);
    }

    #[test]
    fn memory() {
        let deque = Arc::new(Deque::<String>::new());
        let num_threads = 4;
        let num_vals = 32;
        for i in 0 .. (num_threads * num_vals) {
            let s = format!("sro{:03}orz", i);
            deque.push_back(s);
        }
        let handles: Vec<_> = (0 .. num_threads)
            .map(|k| {
                let dq = Arc::clone(&deque);
                thread::spawn(move || {
                    let mut vals = Vec::new();
                    for _ in 0 .. num_vals {
                        // println!("[T{}] Popping", k);
                        let res = dq.pop_back();
                        match res {
                            Some(v) => {
                                // println!("  [T{}] Got: {}", k, v);
                                vals.push(v);
                            },
                            None => panic!("[T{}] Failed", k),
                        }
                    }
                    vals
                })
            })
            .collect();
        let mut all_vals = Vec::new();
        for (i, handle) in handles.into_iter().enumerate() {
            let vec = handle.join().unwrap();
            println!("Values (T{}): {:?}", i, vec);
            all_vals.extend(vec);
        }
        all_vals.sort_unstable();
        let expected: Vec<_> = (0 .. (num_threads * num_vals))
            .map(|i| format!("sro{:03}orz", i))
            .collect();
        assert_eq!(expected, all_vals);
    }

    #[test]
    fn push_back_pop_front() {
        let deque = Deque::<i32>::new();
        assert_eq!(None, deque.pop_front());

        deque.push_back(42);
        assert_eq!(Some(42), deque.pop_front());
        assert_eq!(None, deque.pop_front());

        deque.push_back(43);
        deque.push_back(44);
        assert_eq!(Some(43), deque.pop_front());
        assert_eq!(Some(44), deque.pop_front());
        assert_eq!(None, deque.pop_front());

        deque.push_back(45);
        deque.push_back(46);
        assert_eq!(Some(45), deque.pop_front());
        deque.push_back(47);
        deque.push_back(48);
        assert_eq!(Some(46), deque.pop_front());
        assert_eq!(Some(47), deque.pop_front());
        assert_eq!(Some(48), deque.pop_front());
        assert_eq!(None, deque.pop_front());
    }

    #[test]
    fn concurrent_push_front_pop_back() {
        let deque = Arc::new(Deque::<i32>::new());
        let num_threads: usize = 4;
        let num_vals: usize = 32;
        let push_handles: Vec<_> = (0 .. num_threads)
            .map(|k| {
                let dq = Arc::clone(&deque);
                thread::spawn(move || {
                    for i in 0 .. num_vals {
                        let x = (k * num_vals + i) as i32;
                        // println!("[T{}] Pushing: {}", k, x);
                        dq.push_front(x);
                        // println!("  [T{}] Pushed: {}", k, x);
                    }
                })
            })
            .collect();
        let pop_handles: Vec<_> = (0 .. num_threads)
            .map(|_k| {
                let dq = Arc::clone(&deque);
                thread::spawn(move || {
                    let mut vals = Vec::new();
                    while vals.len() < num_vals {
                        // println!("[T{}] Popping", k);
                        if let Some(v) = dq.pop_back() {
                            // println!("  [T{}] Got: {}", k, v);
                            vals.push(v);
                        } else {
                            // println!("  [T{}] Got nothing", k);
                        }
                    }
                    vals
                })
            })
            .collect();
        let mut all_vals = Vec::new();
        push_handles.into_iter().for_each(|h| h.join().unwrap());
        for (i, handle) in pop_handles.into_iter().enumerate() {
            let vals = handle.join().unwrap();
            println!("Values (T{}): {:?}", i, vals);
            all_vals.extend(vals);
        }
        all_vals.sort_unstable();
        let expected = Vec::from_iter(0 .. ((num_threads * num_vals) as i32));
        assert_eq!(expected, all_vals);
    }
}

// ----

// Code from Stack as a reference of TODOs.

/*
impl<T> Stack<T> {
    /// Creates an iterator over `T`s, based on [`pop`](Stack::pop) operation of
    /// the [`Stack`].
    pub fn pop_iter<'stack>(&'stack self) -> PopIter<'stack, T> {
        PopIter { stack: self }
    }

    /// Pushes elements from the given iterable. Acts just like
    /// [`Extend::extend`] but does not require mutability.
    pub fn extend<I>(&self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        for elem in iterable {
            self.push(elem);
        }
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
    }
}

impl<T> Iterator for Stack<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let top = self.top.get_mut();

        NonNull::new(*top).map(|nnptr| {
            // This is safe because we only store pointers allocated via
            // `OwnedAlloc`. Also, we have exclusive access to this pointer.
            let mut node = unsafe { OwnedAlloc::from_raw(nnptr) };
            *top = node.next;
            // This read is we never drop the inner value when dropping the
            // node.
            unsafe { (&mut *node.val as *mut T).read() }
        })
    }
}

impl<T> Extend<T> for Stack<T> {
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        (&*self).extend(iterable)
    }
}

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let this = Self::new();
        this.extend(iterable);
        this
    }
}

/// An iterator based on [`pop`](Stack::pop) operation of the [`Stack`].
pub struct PopIter<'stack, T>
where
    T: 'stack,
{
    stack: &'stack Stack<T>,
}

impl<'stack, T> Iterator for PopIter<'stack, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<'stack, T> fmt::Debug for PopIter<'stack, T> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "PopIter {} stack: {:?} {}", '{', self.stack, '}')
    }
}
*/
