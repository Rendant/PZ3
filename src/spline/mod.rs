mod linear;
mod smoothing;

pub use linear::LinearSpline;
pub use smoothing::SmoothingSpline;

// Общий интерфейс сплайна
pub trait Spline {
    // Значение сплайна
    fn value(&self, x: f64) -> f64;
}

// Номер i отрезка [nodes[i], nodes[i+1]], содержащего x.
fn segment_index(nodes: &[f64], x: f64) -> usize {
    let last = nodes.len() - 1;

    // Бинарный поиск: количество узлов, не превосходящих x
    nodes.partition_point(|&node| node <= x).clamp(1, last) - 1
}
