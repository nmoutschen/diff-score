use std::collections::HashSet;

use serde_json::{Map, Value};

use crate::DiffScore;

impl DiffScore for Value {
    fn diff_score(&self, other: &Self) -> f64 {
        match (self, other) {
            (Value::Array(self_arr), Value::Array(other_arr)) => self_arr.diff_score(other_arr),
            (Value::Bool(self_bool), Value::Bool(other_bool)) => self_bool.diff_score(other_bool),
            (Value::Null, Value::Null) => 0.0,
            (Value::Number(self_num), Value::Number(other_num)) => {
                self_num.as_f64().diff_score(&other_num.as_f64())
            }
            (Value::Object(self_obj), Value::Object(other_obj)) => self_obj.diff_score(other_obj),
            (Value::String(self_s), Value::String(other_s)) => self_s.diff_score(other_s),
            (_, _) => 1.0,
        }
    }
}

impl DiffScore for Map<String, Value> {
    fn diff_score(&self, other: &Self) -> f64 {
        let mut seen: HashSet<&String> = HashSet::new();

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
