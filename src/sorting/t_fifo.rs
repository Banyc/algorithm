//! - source: <https://github.com/torvalds/linux/blob/master/net/sched/sch_netem.c>

use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TFifo<K, V> {
    root: BTreeMap<K, V>,
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
    K: Ord,
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

        self.root.insert(key, value);
    }

    pub fn pop(&mut self) -> Option<(K, V)> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;

        if let Some(root_first) = self.root.pop_first() {
            return Some(root_first);
        };

        self.list.pop_front()
    }

    pub fn peak(&self) -> Option<(&K, &V)> {
        if self.len == 0 {
            return None;
        }

        if let Some(root_first) = self.root.first_key_value() {
            return Some(root_first);
        };

        self.list.front().map(|(k, v)| (k, v))
    }
}

impl<K, V> Default for TFifo<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_order() {
        let mut fifo = TFifo::default();

        fifo.insert(1, 1);
        fifo.insert(2, 2);
        fifo.insert(3, 3);

        assert_eq!(fifo.len(), 3);

        assert_eq!(fifo.peak(), Some((&1, &1)));
        assert_eq!(fifo.pop(), Some((1, 1)));
        assert_eq!(fifo.peak(), Some((&2, &2)));
        assert_eq!(fifo.pop(), Some((2, 2)));
        assert_eq!(fifo.peak(), Some((&3, &3)));
        assert_eq!(fifo.pop(), Some((3, 3)));
        assert_eq!(fifo.peak(), None);
        assert_eq!(fifo.pop(), None);

        assert!(fifo.is_empty());
    }

    #[test]
    fn disorder() {
        let mut fifo = TFifo::default();

        fifo.insert(3, 3);
        fifo.insert(1, 1);
        fifo.insert(2, 2);

        assert_eq!(fifo.len(), 3);

        assert_eq!(fifo.peak(), Some((&1, &1)));
        assert_eq!(fifo.pop(), Some((1, 1)));
        assert_eq!(fifo.peak(), Some((&2, &2)));
        assert_eq!(fifo.pop(), Some((2, 2)));
        assert_eq!(fifo.peak(), Some((&3, &3)));
        assert_eq!(fifo.pop(), Some((3, 3)));
        assert_eq!(fifo.peak(), None);
        assert_eq!(fifo.pop(), None);

        assert!(fifo.is_empty());
    }
}
