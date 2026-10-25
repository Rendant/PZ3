import numpy as np
from scipy.interpolate import make_interp_spline, make_smoothing_spline

# Параметры выборки
SAMPLE_SIZE = 1733
MEAN = 0.96
STD_DEV = 3.54
SEED = 42

# Исследуемые значения параметра сглаживания p
SMOOTH_VALUES = [0.0, 0.2, 0.4, 0.6, 0.8, 0.99]


def main():
    # Генерация выборки
    rng = np.random.default_rng(SEED)
    x = np.arange(1, SAMPLE_SIZE + 1, dtype=float)
    y = rng.normal(MEAN, STD_DEV, SAMPLE_SIZE)

    print(f"Выборочное среднее = {y.mean():.4f}, выборочное СКО = {y.std(ddof=1):.4f}\n")

    # Интерполяционный линейный сплайн
    interpolation = make_interp_spline(x, y, k=1)

    print(f"{'p':>5} {'lam':>10} {'||f - g||^2':>14}")

    for p in SMOOTH_VALUES:
        lam = p / (1 - p)
        s = make_smoothing_spline(x, y, lam=lam)

        print(f"{p:5} {lam:10.4f} {np.sum((s(x) - y) ** 2):14.4f}")

    # Без lam параметр сглаживания подбирается автоматически по критерию GCV
    s = make_smoothing_spline(x, y)
    print(f"{'GCV':>5} {'?':>10} {np.sum((s(x) - y) ** 2):14.4f}")

    print(f"\nИнтерполяционный сплайн: max|f - g| = {np.max(np.abs(interpolation(x) - y)):.2e}")


if __name__ == "__main__":
    main()
