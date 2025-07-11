use std::{rc::Rc, sync::Arc};

use crate::DiffScore;

macro_rules! impl_eq {
    ($type:ty) => {
        impl DiffScore for $type {
            fn diff_score(&self, other: &Self) -> f64 {
                if self != other { 1.0 } else { 0.0 }
            }
        }
    };
}

impl_eq!(bool);
impl_eq!(char);
impl_eq!(f32);
impl_eq!(f64);
impl_eq!(i128);
impl_eq!(i16);
impl_eq!(i32);
impl_eq!(i64);
impl_eq!(i8);
impl_eq!(isize);
impl_eq!(std::ffi::OsStr);
impl_eq!(std::ffi::OsString);
impl_eq!(std::net::IpAddr);
impl_eq!(std::net::Ipv4Addr);
impl_eq!(std::net::Ipv6Addr);
impl_eq!(std::path::Path);
impl_eq!(std::path::PathBuf);
impl_eq!(std::time::Duration);
impl_eq!(std::time::Instant);
impl_eq!(std::time::SystemTime);
impl_eq!(&str);
impl_eq!(String);
impl_eq!(u128);
impl_eq!(u16);
impl_eq!(u32);
impl_eq!(u64);
impl_eq!(u8);
impl_eq!(usize);

macro_rules! impl_deref {
    ($type:ident) => {
        impl<T> DiffScore for $type<T>
        where
            T: DiffScore,
        {
            fn diff_score(&self, other: &Self) -> f64 {
                (**self).diff_score(&**other)
            }
        }
    };
}

impl_deref!(Box);
impl_deref!(Rc);
impl_deref!(Arc);

impl<T> DiffScore for Option<T>
where
    T: DiffScore,
{
    fn diff_score(&self, other: &Self) -> f64 {
        match (self, other) {
            (Some(self_val), Some(other_val)) => self_val.diff_score(other_val),
            (None, None) => 0.0,
            (_, _) => 1.0,
        }
    }
}

impl<T, E> DiffScore for Result<T, E>
where
    T: DiffScore,
    E: DiffScore,
{
    fn diff_score(&self, other: &Self) -> f64 {
        match (self, other) {
            (Ok(self_val), Ok(other_val)) => self_val.diff_score(other_val),
            (Err(self_err), Err(other_err)) => self_err.diff_score(other_err),
            (_, _) => 1.0,
        }
    }
}

impl<T> DiffScore for &T
where
    T: DiffScore,
{
    fn diff_score(&self, other: &Self) -> f64 {
        (**self).diff_score(&**other)
    }
}
