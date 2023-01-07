const QUANTIZE_NEAREST: f32 = 0.0001;

#[macro_export]
macro_rules! string_ref {
    ($s:expr) => {
        StringRef::new($s).unwrap_unchecked()
    };
}

// FIXME: Can be possible reason of unexpected behaviour.
#[inline(always)]
pub fn quantize(value: f32) -> f32 {
    QUANTIZE_NEAREST * (value / QUANTIZE_NEAREST).round()
}
