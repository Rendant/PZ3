use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::{Distribution, Normal};

// Выборка случайных чисел
pub struct Sample {
    x: Vec<f64>,
    y: Vec<f64>,
}

impl Sample {
    // Выборка из нормального распределения
    pub fn normal(size: usize, mean: f64, std_dev: f64, seed: u64) -> Self {
        let distribution = Normal::new(mean, std_dev).expect("Некорректные параметры нормального распределения");

        let mut rng = StdRng::seed_from_u64(seed);

        Self {
            x: (1..=size).map(|i| i as f64).collect(),
            y: (0..size).map(|_| distribution.sample(&mut rng)).collect(),
        }
    }

    pub fn x(&self) -> &[f64] {
        &self.x
    }

    pub fn y(&self) -> &[f64] {
        &self.y
    }

    pub fn len(&self) -> usize {
        self.x.len()
    }

    // Выборочное среднее
    pub fn mean(&self) -> f64 {
        self.y.iter().sum::<f64>() / self.len() as f64
    }

    // Исправленное выборочное стандартное отклонение
    pub fn std_dev(&self) -> f64 {
        let mean = self.mean();

        let sum: f64 = self.y.iter().map(|y| (y - mean).powi(2)).sum();

        (sum / (self.len() - 1) as f64).sqrt()
    }
}
