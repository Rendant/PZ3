use super::{segment_index, Spline};

// Интерполяционный линейный сплайн
pub struct LinearSpline {
    x: Vec<f64>,
    y: Vec<f64>,
}

impl LinearSpline {
    pub fn new(x: &[f64], y: &[f64]) -> Self {
        Self {
            x: x.to_vec(),
            y: y.to_vec(),
        }
    }

    // Наклон сплайна на отрезке k
    fn slope(&self, k: usize) -> f64 {
        (self.y[k + 1] - self.y[k]) / (self.x[k + 1] - self.x[k])
    }
}

impl Spline for LinearSpline {
    fn value(&self, x: f64) -> f64 {
        let k = segment_index(&self.x, x);

        let t = x - self.x[k];

        self.y[k] + self.slope(k) * t
    }
}
