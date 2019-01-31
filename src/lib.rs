#![unstable(feature = "stdsimd", issue = "27731")]
#![feature(staged_api)]

#[stable(feature = "simd_x86", since = "1.27.0")]
#[macro_export]
macro_rules! is_x86_feature_detected {
    ("foo") => { $crate::check($crate::Feature::foo) };
    ("bar") => { $crate::check($crate::Feature::bar) };
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[doc(hidden)]
#[stable(feature = "simd_x86", since = "1.27.0")]
// #[unstable(feature = "stdsimd_internal", issue = "0")]
pub enum Feature {
    #[stable(feature = "simd_x86", since = "1.27.0")] foo,
    #[unstable(feature = "mmx_target_feature", issue = "44839")] bar,
}

#[stable(feature = "simd_x86", since = "1.27.0")]
pub fn check(_: Feature) -> bool { true }

/// Check that MMX fails without features
///
/// ```rust,compile_fail
/// #![feature(stdsimd)]
/// #[macro_use(is_x86_feature_detected)]
/// extern crate test_detect;
///
/// fn main() {
///    let _ = is_x86_feature_detected!("bar");
/// }
/// ```
#[unstable(feature = "mmx_target_feature", issue = "44839")]
#[allow(dead_code)]
fn mmx_fails() {}

/// Check that MMX passes when the feature is enabled
///
/// ```rust
/// #![feature(stdsimd, mmx_target_feature)]
/// #[macro_use(is_x86_feature_detected)]
/// extern crate test_detect;
///
/// fn main() {
///    let _ = is_x86_feature_detected!("bar");
/// }
/// ```
#[unstable(feature = "mmx_target_feature", issue = "44839")]
#[allow(dead_code)]
fn mmx_passes() {}


/// Check that MMX passes when variant without features is uses
///
/// ```rust
/// #![feature(stdsimd)]
/// #[macro_use(is_x86_feature_detected)]
/// extern crate test_detect;
///
/// fn main() {
///    let _ = is_x86_feature_detected!("foo");
/// }
/// ```
#[unstable(feature = "mmx_target_feature", issue = "44839")]
#[allow(dead_code)]
fn mmx_passes2() {}
