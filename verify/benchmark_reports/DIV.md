# MathDivide benchmark (`DIV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 189.39M | 0.003 | 291.11M | 0.030 | 5.75× | 8.84× |
| 10,000 | 0.012 | 855.92M | 0.009 | 1.17G | 0.036 | 3.11× | 4.24× |
| 100,000 | 0.084 | 1.20G | 0.054 | 1.84G | 0.087 | 1.04× | 1.60× |
| 1,000,000 | 1.451 | 689.15M | 0.965 | 1.04G | 1.080 | 0.74× | 1.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.161 | 2.12× |
| 1 | 5 | 0.280 | 0.569 | 2.03× |
| 1 | 10 | 0.586 | 1.219 | 2.08× |
| 10 | 1 | 0.055 | 0.105 | 1.88× |
| 10 | 5 | 0.272 | 0.529 | 1.94× |
| 10 | 10 | 0.569 | 1.106 | 1.94× |
| 100 | 1 | 0.055 | 0.102 | 1.83× |
| 100 | 5 | 0.296 | 0.531 | 1.79× |
| 100 | 10 | 0.659 | 1.132 | 1.72× |
| 1,000 | 1 | 0.077 | 0.128 | 1.66× |
| 1,000 | 5 | 0.288 | 0.538 | 1.87× |
| 1,000 | 10 | 0.610 | 1.360 | 2.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
