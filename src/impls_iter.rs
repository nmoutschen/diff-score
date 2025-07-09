use std::{
    collections::{BTreeSet, HashMap, HashSet},
    hash::Hash,
};

use crate::DiffScore;

const DEFAULT_MATCH_WINDOW: usize = 5;
const DEFAULT_ORDER_PENALTY: f64 = 1.0;

/// Calculate diff score for ordered slices
fn diff_score_ordered<T: DiffScore>(a: &[T], b: &[T], window: usize, order_penalty: f64) -> f64 {
    let mut used = vec![false; b.len()];

    let self_score: f64 = a
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let mut best_score = item.missing_score();
            let mut best_j = None;

            let start = i.saturating_sub(window);
            let end = (i + window + 1).min(b.len());

            // Finding the item in other with the smallest matching value
            for j in start..end {
                // Already used this value, skipping
                if used[j] {
                    continue;
                }

                let score = item.diff_score(&b[j]) + ((j as f64 - i as f64).abs() * order_penalty);
                if score < best_score {
                    best_score = score;
                    best_j = Some(j);
                }
            }

            if let Some(j) = best_j {
                used[j] = true;
            }

            best_score
        })
        .sum();

    let other_score: f64 = b
        .iter()
        .enumerate()
        .filter_map(|(j, item)| (!used[j]).then_some(item.missing_score()))
        .sum();

    self_score + other_score
}

macro_rules! impl_str {
    ($type:ty, $method:ident) => {
        impl DiffScore for $type {
            fn diff_score(&self, other: &Self) -> f64 {
                diff_score_ordered(
                    self.$method(),
                    other.$method(),
                    DEFAULT_MATCH_WINDOW,
                    DEFAULT_ORDER_PENALTY,
                )
            }

            fn missing_score(&self) -> f64 {
                self.$method().diff_score(&Default::default())
            }
        }
    };
}

impl_str!(str, as_bytes);
impl_str!(String, as_bytes);
impl_str!(std::ffi::CStr, to_bytes);
impl_str!(std::ffi::CString, to_bytes);

impl<T> DiffScore for &[T]
where
    T: DiffScore,
{
    fn diff_score(&self, other: &Self) -> f64 {
        diff_score_ordered(self, other, DEFAULT_MATCH_WINDOW, DEFAULT_ORDER_PENALTY)
    }
}

impl<T> DiffScore for Vec<T>
where
    T: DiffScore,
{
    fn diff_score(&self, other: &Self) -> f64 {
        diff_score_ordered(self, other, DEFAULT_MATCH_WINDOW, DEFAULT_ORDER_PENALTY)
    }
}

impl<T> DiffScore for BTreeSet<T>
where
    T: DiffScore + Ord,
{
    fn diff_score(&self, other: &Self) -> f64 {
        let self_score: f64 = self
            .iter()
            .filter_map(|item| (!other.contains(item)).then_some(item.missing_score()))
            .sum();
        let other_score: f64 = other
            .iter()
            .filter_map(|item| (!self.contains(item)).then_some(item.missing_score()))
            .sum();
        self_score + other_score
    }
}

impl<T> DiffScore for HashSet<T>
where
    T: DiffScore + Eq + Hash,
{
    fn diff_score(&self, other: &Self) -> f64 {
        let self_score: f64 = self
            .iter()
            .filter_map(|item| (!other.contains(item)).then_some(item.missing_score()))
            .sum();
        let other_score: f64 = other
            .iter()
            .filter_map(|item| (!self.contains(item)).then_some(item.missing_score()))
            .sum();
        self_score + other_score
    }
}

impl<K, T> DiffScore for HashMap<K, T>
where
    K: Eq + Hash,
    T: DiffScore,
{
    fn diff_score(&self, other: &Self) -> f64 {
        let mut seen: HashSet<&K> = HashSet::new();

        let self_score = self
            .iter()
            .map(|(key, item)| match other.get(key) {
                Some(other_item) => {
                    seen.insert(key);
                    item.diff_score(other_item)
                }
                None => item.missing_score(),
            })
            .sum::<f64>();

        let other_score: f64 = other
            .iter()
            .filter_map(|(key, item)| (!seen.contains(key)).then_some(item.missing_score()))
            .sum();

        self_score + other_score
    }
}
