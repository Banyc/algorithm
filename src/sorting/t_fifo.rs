//! - source: <https://github.com/torvalds/linux/blob/master/net/sched/sch_netem.c>

use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TFifo<K, V> {
    root: BTreeMap<K, VecDeque<V>>,
    list: VecDeque<(K, V)>,
    len: usize,
}

impl<K, V> TFifo<K, V> {
    pub fn new() -> Self {
        TFifo {
            root: BTreeMap::new(),
            list: VecDeque::new(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<K, V> TFifo<K, V>
where
    K: Ord + Clone,
{
    pub fn insert(&mut self, key: K, value: V) {
        self.len += 1;

        let list_tail = match self.list.back() {
            Some((k, v)) => (k, v),
            None => {
                self.list.push_back((key, value));
                return;
            }
        };

        if key >= *list_tail.0 {
            self.list.push_back((key, value));
            return;
        }

        let root_entry = self.root.entry(key).or_insert_with(VecDeque::new);
        root_entry.push_back(value);
    }

    pub fn pop(&mut self) -> Option<(K, V)> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;

        // Pop the first value from the list if there are no root entries
        let mut first_entry = match self.root.first_entry() {
            Some(entry) => entry,
            None => return self.list.pop_front(),
        };

        // Pop the first value from the list if it is smaller than the first root entry
        if let Some((k, _)) = self.list.front() {
            if k < first_entry.key() {
                return self.list.pop_front();
            }
        }

        // Pop the first value from the first root entry
        let key = first_entry.key().clone();
        let values = first_entry.get_mut();
        let value = values.pop_front().unwrap();

        // Remove the root entry if it is empty
        if values.is_empty() {
            self.root.remove(&key);
        }

        Some((key, value))
    }

    pub fn peek(&self) -> Option<(&K, &V)> {
        if self.len == 0 {
            return None;
        }

        // Peek the first value from the list if there are no root entries
        let (key, values) = match self.root.first_key_value() {
            Some(entry) => entry,
            None => return self.list.front().map(|(k, v)| (k, v)),
        };

        // Peek the first value from the list if it is smaller than the first root entry
        if let Some((k, _)) = self.list.front() {
            if k < key {
                return self.list.front().map(|(k, v)| (k, v));
            }
        }

        // Peek the first value from the first root entry
        Some((key, values.front().unwrap()))
    }
}

impl<K, V> Default for TFifo<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use super::*;

    #[test]
    fn in_order() {
        let mut fifo = TFifo::default();

        fifo.insert(1, 1);
        fifo.insert(2, 2);
        fifo.insert(2, 22);
        fifo.insert(3, 3);

        assert_eq!(fifo.len(), 4);

        assert_eq!(fifo.peek(), Some((&1, &1)));
        assert_eq!(fifo.pop(), Some((1, 1)));
        assert_eq!(fifo.peek(), Some((&2, &2)));
        assert_eq!(fifo.pop(), Some((2, 2)));
        assert_eq!(fifo.peek(), Some((&2, &22)));
        assert_eq!(fifo.pop(), Some((2, 22)));
        assert_eq!(fifo.peek(), Some((&3, &3)));
        assert_eq!(fifo.pop(), Some((3, 3)));
        assert_eq!(fifo.peek(), None);
        assert_eq!(fifo.pop(), None);

        assert!(fifo.is_empty());
    }

    #[test]
    fn disorder() {
        let mut fifo = TFifo::default();

        fifo.insert(3, 3);
        fifo.insert(1, 1);
        fifo.insert(1, 11);
        fifo.insert(2, 2);

        assert_eq!(fifo.len(), 4);

        assert_eq!(fifo.peek(), Some((&1, &1)));
        assert_eq!(fifo.pop(), Some((1, 1)));
        assert_eq!(fifo.peek(), Some((&1, &11)));
        assert_eq!(fifo.pop(), Some((1, 11)));
        assert_eq!(fifo.peek(), Some((&2, &2)));
        assert_eq!(fifo.pop(), Some((2, 2)));
        assert_eq!(fifo.peek(), Some((&3, &3)));
        assert_eq!(fifo.pop(), Some((3, 3)));
        assert_eq!(fifo.peek(), None);
        assert_eq!(fifo.pop(), None);

        assert!(fifo.is_empty());
    }

    #[test]
    fn disorder_2() {
        let mut fifo = TFifo::default();

        fifo.insert(1, 1);
        fifo.insert(3, 3);
        fifo.insert(2, 2);

        assert_eq!(fifo.len(), 3);

        assert_eq!(fifo.peek(), Some((&1, &1)));
        assert_eq!(fifo.pop(), Some((1, 1)));
        assert_eq!(fifo.peek(), Some((&2, &2)));
        assert_eq!(fifo.pop(), Some((2, 2)));
        assert_eq!(fifo.peek(), Some((&3, &3)));
        assert_eq!(fifo.pop(), Some((3, 3)));
        assert_eq!(fifo.peek(), None);
        assert_eq!(fifo.pop(), None);

        assert!(fifo.is_empty());
    }

    const N: usize = 1000;

    #[bench]
    fn bench_in_order(b: &mut test::Bencher) {
        let mut fifo = TFifo::default();

        b.iter(|| {
            for i in 0..N {
                let fifo = black_box(&mut fifo);
                fifo.insert(black_box(i), black_box(i));
            }
            for _ in 0..N {
                let fifo = black_box(&mut fifo);
                black_box(fifo.pop());
            }
        });
    }

    #[bench]
    fn bench_vecdeqeue_in_order(b: &mut test::Bencher) {
        let mut vecd = VecDeque::default();

        b.iter(|| {
            for i in 0..N {
                let vecd = black_box(&mut vecd);
                vecd.push_back(black_box((i, i)));
            }
            for _ in 0..N {
                let vecd = black_box(&mut vecd);
                black_box(vecd.pop_front());
            }
        });
    }

    #[bench]
    fn bench_disorder(b: &mut test::Bencher) {
        let mut fifo = TFifo::default();

        b.iter(|| {
            for i in (0..N).rev() {
                let fifo = black_box(&mut fifo);
                fifo.insert(black_box(i), black_box(i));
            }
            for _ in 0..N {
                let fifo = black_box(&mut fifo);
                black_box(fifo.pop());
            }
        });
    }
}
