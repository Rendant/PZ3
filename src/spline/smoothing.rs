use super::{segment_index, Spline};

// Сглаживающий сплайн
pub struct SmoothingSpline {
    // Параметр сглаживания
    smooth: f64,
    // Узлы сплайна
    points: Vec<f64>,
    // Коэффициенты разложения по базису
    alpha: Vec<f64>,
}

impl SmoothingSpline {
    // Вес точки в дискретном скалярном произведении
    pub const WEIGHT: f64 = 1.0;

    pub fn new(smooth: f64) -> Self {
        Self {
            smooth,
            points: Vec::new(),
            alpha: Vec::new(),
        }
    }

    // Переход на [-1, 1]:
    fn to_master_element(&self, seg_num: usize, x: f64) -> f64 {
        2.0 * (x - self.points[seg_num]) / (self.points[seg_num + 1] - self.points[seg_num]) - 1.0
    }

    // Базисные функции на [-1, 1]:
    fn basis_function(number: usize, ksi: f64) -> f64 {
        match number {
            1 => 0.5 * (1.0 - ksi),
            2 => 0.5 * (1.0 + ksi),
            _ => panic!("Неверный номер базисной функции"),
        }
    }

    // Построение сплайна по точкам points и значениям функции f_value
    pub fn update(&mut self, points: &[f64], f_value: &[f64]) {
        self.points = points.to_vec();

        // Число отрезков разбиения
        let num_segments = points.len() - 1;

        // Правая часть
        let mut alpha = vec![0.0; num_segments + 1];

        // Диагонали матрицы
        let mut a = vec![0.0; num_segments + 1];
        let mut b = vec![0.0; num_segments + 1];
        let mut c = vec![0.0; num_segments + 1];

        let smooth = self.smooth;

        // Сборка вклада точки в СЛАУ:
        // i - номер отрезка, x - точка, f_val - значение функции в точке, w - вес
        let mut assembling = |i: usize, x: f64, f_val: f64, w: f64| {
            // Переход на мастер-элемент
            let ksi = self.to_master_element(i, x);

            // Значения базисных функций на мастер-элементе
            let f1 = Self::basis_function(1, ksi);
            let f2 = Self::basis_function(2, ksi);

            // Вклад в матрицу
            b[i] += (1.0 - smooth) * w * f1 * f1;
            b[i + 1] += (1.0 - smooth) * w * f2 * f2;
            a[i + 1] += (1.0 - smooth) * w * f1 * f2;
            c[i] += (1.0 - smooth) * w * f2 * f1;

            // Вклад в правую часть
            alpha[i] += (1.0 - smooth) * w * f1 * f_val;
            alpha[i + 1] += (1.0 - smooth) * w * f2 * f_val;
        };

        // Сборка СЛАУ
        for i in 0..num_segments {
            // Вклад точек в СЛАУ
            let w = Self::WEIGHT;

            assembling(i, points[i], f_value[i], w);
            assembling(i, points[i + 1], f_value[i + 1], w);
        }

        // Вклад слагаемого с производной
        for i in 0..num_segments {
            let h = points[i + 1] - points[i];

            b[i] += 1.0 / h * smooth;
            b[i + 1] += 1.0 / h * smooth;
            a[i + 1] -= 1.0 / h * smooth;
            c[i] -= 1.0 / h * smooth;
        }

        // Метод прогонки: прямой ход
        for j in 1..=num_segments {
            // Диагональ
            b[j] -= a[j] / b[j - 1] * c[j - 1];

            // Правая часть
            alpha[j] -= a[j] / b[j - 1] * alpha[j - 1];
        }

        // Метод прогонки: обратный ход
        alpha[num_segments] /= b[num_segments];

        for j in (0..num_segments).rev() {
            alpha[j] = (alpha[j] - alpha[j + 1] * c[j]) / b[j];
        }

        self.alpha = alpha;
    }

    // Квадрат нормы производной
    pub fn derivative_norm_squared(&self) -> f64 {
        self.points
            .windows(2)
            .zip(self.alpha.windows(2))
            .map(|(x, a)| (a[1] - a[0]).powi(2) / (x[1] - x[0]))
            .sum()
    }
}

impl Spline for SmoothingSpline {
    fn value(&self, x: f64) -> f64 {
        let i = segment_index(&self.points, x);

        // Переход на мастер-элемент
        let ksi = self.to_master_element(i, x);

        self.alpha[i] * Self::basis_function(1, ksi) + self.alpha[i + 1] * Self::basis_function(2, ksi)
    }
}
