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
impl_eq!(std::ffi::CStr);
impl_eq!(std::ffi::CString);
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
impl_eq!(str);
impl_eq!(String);

macro_rules! impl_num {
    ($type:ty) => {
        impl DiffScore for $type {
            fn diff_score(&self, other: &Self) -> f64 {
                (*self as f64 - *other as f64).abs()
            }
        }
    };
}

impl_num!(u8);
impl_num!(u16);
impl_num!(u32);
impl_num!(u64);
impl_num!(u128);
impl_num!(usize);
impl_num!(i8);
impl_num!(i16);
impl_num!(i32);
impl_num!(i64);
impl_num!(i128);
impl_num!(isize);
impl_num!(f32);
impl_num!(f64);
