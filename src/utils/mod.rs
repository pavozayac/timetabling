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

pub fn is_subset<T>(superset: T, iterator: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut hashtable = HashSet::new();

    for s in superset {
        hashtable.insert(s);
    }

    iterator.into_iter().all(move |e| hashtable.insert(e))
}
