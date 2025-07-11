mod impls;

pub use diff_score_derive::DiffScore;

pub trait DiffScore {
    fn diff_score(&self, other: &Self) -> f64;
}
