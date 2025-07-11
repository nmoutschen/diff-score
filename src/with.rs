use std::{collections::HashSet, hash::Hash};

use crate::DiffScore;

/// Returns 0.0 if the two values are equal, 1.0 otherwise
pub fn eq<T: PartialEq>(a: &T, b: &T) -> f64 {
    if a == b { 0.0 } else { 1.0 }
}

/// Returns 0.0 if the discriminants of an enum match, 1.0 otherwise
pub fn discriminant<T>(a: &T, b: &T) -> f64 {
    if std::mem::discriminant(a) == std::mem::discriminant(b) {
        0.0
    } else {
        1.0
    }
}

/// Compare two iterators as a set of items
///
/// ## Set matching
///
/// This function creates a union of all items in the two sets, adds 1 point for every item not
/// present in both sets, then divides that by the number of distinct elements.
///
/// For example, if you have a set `[a, b]` and `[a, c]`, this function will compare the union
/// of these two sets `[a, b, c]`. It will notice that `b` is not present in the second set and
/// `c` is not present in the first one. This means two elements don't match out of three, thus
/// it will return a score of `2.0 / 3.0` (2 items not matching over three distinct items).
pub fn set<T>(a: T, b: T) -> f64
where
    T: IntoIterator,
    T::Item: DiffScore + Hash + Eq,
{
    let set_a: HashSet<T::Item> = a.into_iter().collect();
    let set_b: HashSet<T::Item> = b.into_iter().collect();

    if set_a.is_empty() && set_b.is_empty() {
        return 0.0;
    }

    let all_items = set_a.union(&set_b);
    let mut score = 0.0;
    let mut total = 0.0;
    for item in all_items {
        total += 1.0;
        if !(set_a.get(item).is_some() && set_b.get(item).is_some()) {
            score += 1.0;
        }
    }

    score / total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case(vec![], vec![], 0.0)]
    #[case(vec!["a"], vec!["a"], 0.0)]
    #[case(vec!["a"], vec!["b"], 1.0)]
    #[case(vec!["a"], vec!["a", "b"], 0.5)]
    #[case(vec!["a", "b"], vec!["a", "c"], 2./3.)]
    #[case(vec!["a", "b"], vec!["a", "b"], 0.0)]
    #[case(vec!["a", "b"], vec!["c", "d"], 1.0)]
    #[case(vec!["a", "b"], vec!["a", "b", "c"], 1./3.)]
    #[case(vec!["a", "b", "c"], vec!["a", "b", "d"], 0.5)]
    fn test_set_vec(#[case] a: Vec<&str>, #[case] b: Vec<&str>, #[case] expected: f64) {
        assert_eq!(set(&a, &b), expected);
        assert_eq!(set(&b, &a), expected);
    }
}
