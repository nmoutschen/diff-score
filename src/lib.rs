mod impls;
mod impls_iter;
#[cfg(feature = "serde_json")]
mod impls_json;

pub use diff_score_derive::DiffScore;

pub trait DiffScore {
    fn diff_score(&self, other: &Self) -> f64;

    /// Missing score when there are no other object to diff against
    fn missing_score(&self) -> f64 {
        1.0
    }
}
