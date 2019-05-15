pub trait InRange {
    fn in_range(&self, begin: Self, end: Self) -> bool;
}

impl InRange for f64 {
    fn in_range(&self, begin: f64, end: f64) -> bool {
        *self >= begin && *self < end
    }
}
