use super::{
    bucket::{self, Bucket, Garbage},
    guard::ReadGuard,
    table::Table,
};
use incin::Pause;
use owned_alloc::OwnedAlloc;
use std::{fmt, mem::replace, ptr::NonNull};

/// An iterator over key-vaue entries of a [`Map`](super::Map). The `Item` of
/// this iterator is a [`ReadGuard`].
#[derive(Debug)]
pub struct Iter<'map, K, V>
where
    K: 'map,
    V: 'map,
{
    pause: Pause<'map, Garbage<K, V>>,
    tables: Vec<&'map Table<K, V>>,
    curr_table: Option<(&'map Table<K, V>, usize)>,
    cache: Vec<ReadGuard<'map, K, V>>,
}

impl<'map, K, V> Iter<'map, K, V> {
    pub(super) fn new(
        pause: Pause<'map, Garbage<K, V>>,
        top: &'map Table<K, V>,
    ) -> Self {
        Self {
            pause,
            tables: Vec::new(),
            curr_table: Some((top, 0)),
            cache: Vec::new(),
        }
    }
}

impl<'map, K, V> Iterator for Iter<'map, K, V> {
    type Item = ReadGuard<'map, K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(guard) = self.cache.pop() {
                break Some(guard);
            }

            let (table, index) = self.curr_table?;
            self.curr_table = match table.load_index(index) {
                Some(ptr) if ptr.is_null() => Some((table, index + 1)),

                Some(ptr) if ptr as usize & 1 == 0 => unsafe {
                    let ptr = ptr as *mut Bucket<K, V>;
                    let mut cache = replace(&mut self.cache, Vec::new());

                    (*ptr).collect(&self.pause, &mut cache);

                    self.cache = cache;
                    Some((table, index + 1))
                },

                Some(ptr) => {
                    let ptr = (ptr as usize & !1) as *mut Table<K, V>;
                    self.tables.push(unsafe { &*ptr });
                    Some((table, index + 1))
                },

                None => self.tables.pop().map(|tbl| (tbl, 0)),
            };
        }
    }
}

unsafe impl<'map, K, V> Send for Iter<'map, K, V>
where
    K: Send,
    V: Send,
{
}

unsafe impl<'map, K, V> Sync for Iter<'map, K, V>
where
    K: Sync,
    V: Sync,
{
}

/// An owned iterator over key-vaue entries of a [`Map`](super::Map).
pub struct IntoIter<K, V> {
    tables: Vec<OwnedAlloc<Table<K, V>>>,
    curr_table: Option<(OwnedAlloc<Table<K, V>>, usize)>,
    entries: bucket::IntoIter<K, V>,
}

impl<K, V> IntoIter<K, V> {
    pub(super) fn new(top: OwnedAlloc<Table<K, V>>) -> Self {
        Self {
            tables: Vec::new(),
            curr_table: Some((top, 0)),
            entries: bucket::IntoIter::empty(),
        }
    }
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(alloc) = self.entries.next() {
                let (pair, _) = alloc.move_inner();
                break Some(pair);
            }

            let (table, index) = self.curr_table.take()?;
            self.curr_table = match table.load_index(index) {
                Some(ptr) if ptr.is_null() => Some((table, index + 1)),

                Some(ptr) if ptr as usize & 1 == 0 => {
                    let ptr = ptr as *mut Bucket<K, V>;
                    let alloc = unsafe {
                        OwnedAlloc::from_raw(NonNull::new_unchecked(ptr))
                    };
                    let (bucket, _) = alloc.move_inner();
                    self.entries = bucket.into_iter();
                    Some((table, index + 1))
                },

                Some(ptr) => {
                    let ptr = (ptr as usize & !1) as *mut Table<K, V>;
                    let alloc = unsafe {
                        OwnedAlloc::from_raw(NonNull::new_unchecked(ptr))
                    };
                    self.tables.push(alloc);
                    Some((table, index + 1))
                },

                None => self.tables.pop().map(|tbl| (tbl, 0)),
            };
        }
    }
}

impl<K, V> Drop for IntoIter<K, V> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
    }
}

impl<K, V> fmt::Debug for IntoIter<K, V> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "IntoIter {} tables: {:?}, curr_table: {:?}, entries: {:?} {}",
            '{', self.tables, self.curr_table, self.entries, '}'
        )
    }
}

unsafe impl<K, V> Send for IntoIter<K, V>
where
    K: Send,
    V: Send,
{
}

unsafe impl<K, V> Sync for IntoIter<K, V>
where
    K: Send,
    V: Send,
{
}

/// An owned iterator over references to key-vaue entries of a
/// [`Map`](super::Map). The reference to the value is mutable (but not the one
/// to the key).
pub struct IterMut<'map, K, V>
where
    K: 'map,
    V: 'map,
{
    tables: Vec<&'map mut Table<K, V>>,
    curr_table: Option<(&'map mut Table<K, V>, usize)>,
    entries: bucket::IterMut<'map, K, V>,
}

impl<'map, K, V> IterMut<'map, K, V> {
    pub(super) fn new(top: &'map mut Table<K, V>) -> Self {
        Self {
            tables: Vec::new(),
            curr_table: Some((top, 0)),
            entries: bucket::IterMut::empty(),
        }
    }
}

impl<'map, K, V> Iterator for IterMut<'map, K, V> {
    type Item = (&'map K, &'map mut V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(refr) = self.entries.next() {
                break Some(refr);
            }

            let (table, index) = self.curr_table.take()?;
            self.curr_table = match table.load_index(index) {
                Some(ptr) if ptr.is_null() => Some((table, index + 1)),

                Some(ptr) if ptr as usize & 1 == 0 => {
                    let ptr = ptr as *mut Bucket<K, V>;
                    let bucket = unsafe { &mut *ptr };
                    self.entries = bucket.into_iter();
                    Some((table, index + 1))
                },

                Some(ptr) => {
                    let ptr = (ptr as usize & !1) as *mut Table<K, V>;
                    self.tables.push(unsafe { &mut *ptr });
                    Some((table, index + 1))
                },

                None => self.tables.pop().map(|tbl| (tbl, 0)),
            };
        }
    }
}

unsafe impl<'map, K, V> Send for IterMut<'map, K, V>
where
    K: Send,
    V: Send,
{
}

unsafe impl<'map, K, V> Sync for IterMut<'map, K, V>
where
    K: Sync,
    V: Sync,
{
}

impl<'map, K, V> fmt::Debug for IterMut<'map, K, V> {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmtr,
            "IntoIter {} tables: {:?}, curr_table: {:?}, entries: {:?} {}",
            '{', self.tables, self.curr_table, self.entries, '}'
        )
    }
}
