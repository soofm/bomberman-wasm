#[derive(PartialEq, Eq)]
pub enum LinearDirection {
    Zero,
    Negative,
    Positive,
}

impl LinearDirection {
    pub fn as_f64(&self) -> f64 {
        match *self {
            LinearDirection::Zero => 0.0,
            LinearDirection::Negative => -1.0,
            LinearDirection::Positive => 1.0,
        }
    }
}

impl Default for LinearDirection {
    fn default() -> Self { LinearDirection::Zero }
}
