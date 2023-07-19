use std::{collections::HashSet, hash::Hash};

pub fn has_unique_items<T>(iterator: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut hashtable = HashSet::new();

    for i in iterator {
        if hashtable.contains(&i) {
            return false;
        }

        hashtable.insert(i);
    }

    true
}

pub fn is_subset<T, U, V>(superset: T, iterator: U) -> bool
where
    T: IntoIterator<Item = V>,
    U: IntoIterator<Item = V>,
    V: Eq + Hash,
{
    let mut set: HashSet<V> = HashSet::new();

    for s in superset {
        set.insert(s);
    }

    iterator.into_iter().all(|e| !set.insert(e))
}
