mod sample;
mod spline;

use sample::Sample;
use spline::{LinearSpline, SmoothingSpline, Spline};

// Параметры выборки
const SAMPLE_SIZE: usize = 1733;
const MEAN: f64 = 0.96;
const STD_DEV: f64 = 3.54;
const SEED: u64 = 42;

// Исследуемые значения параметра сглаживания p
const SMOOTH_VALUES: [f64; 6] = [0.0, 0.2, 0.4, 0.6, 0.8, 0.99];

const TABLE_ROWS: usize = 10;

// Сглаживающий сплайн с параметром p по выборке
fn smoothing_spline(sample: &Sample, p: f64) -> SmoothingSpline {
    let mut s = SmoothingSpline::new(p);

    s.update(sample.x(), sample.y());

    s
}

// Таблица
fn print_table(sample: &Sample, splines: &[(f64, SmoothingSpline)], rows: usize) {
    let mut header = format!("{:>5} | {:>18} | {:>5}", "№", "Случайная величина", "Вес w");

    for (p, _) in splines {
        header += &format!(" | {:>12}", format!("p = {}", p));
    }

    println!("{}", header);

    for i in 0..rows.min(sample.len()) {
        let (x, y) = (sample.x()[i], sample.y()[i]);

        let mut row = format!("{:5} | {:18.9} | {:5}", i + 1, y, SmoothingSpline::WEIGHT);

        for (_, s) in splines {
            row += &format!(" | {:>12.5E}", s.value(x));
        }

        println!("{}", row);
    }
}

// Сумма квадратов отклонений сплайна от выборки и максимальное отклонение
fn residuals(s: &dyn Spline, sample: &Sample) -> (f64, f64) {
    sample
        .x()
        .iter()
        .zip(sample.y())
        .map(|(&x, &y)| (s.value(x) - y).abs())
        .fold((0.0, 0.0), |(sum, max), r| (sum + r * r, f64::max(max, r)))
}

fn main() {
    // Пункт 2. Генерация выборки

    let sample = Sample::normal(SAMPLE_SIZE, MEAN, STD_DEV, SEED);

    println!("Выборка из N({}, {}^2), объём {}", MEAN, STD_DEV, sample.len());

    println!("Выборочное среднее = {:.4}, выборочное СКО = {:.4}\n", sample.mean(), sample.std_dev());

    // Пункт 3. Интерполяционный и сглаживающие сплайны

    let interpolation = LinearSpline::new(sample.x(), sample.y());

    let smoothing: Vec<SmoothingSpline> = SMOOTH_VALUES
        .iter()
        .map(|&p| smoothing_spline(&sample, p))
        .collect();

    println!("{:>7} {:>14} {:>14} {:>14}", "p", "||f - g||^2", "||g'||^2", "max|f - g|");

    for (&p, s) in SMOOTH_VALUES.iter().zip(&smoothing) {
        let (sum, max) = residuals(s, &sample);

        println!("{:7} {:14.6} {:14.6} {:14.6e}", p, sum, s.derivative_norm_squared(), max);
    }

    let (sum, max) = residuals(&interpolation, &sample);

    println!("Интерполяционный сплайн: ||f - g||^2 = {:.2e}, max|f - g| = {:.2e}\n", sum, max);

    // Таблица значений сглаживающего сплайна в точках выборки

    let table_splines: Vec<(f64, SmoothingSpline)> = SMOOTH_VALUES
        .iter()
        .map(|&p| (p, smoothing_spline(&sample, p)))
        .collect();

    print_table(&sample, &table_splines, TABLE_ROWS);
}
