# MathTanh benchmark (`TANH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 163.31M | 0.005 | 198.67M | 0.034 | 5.61× | 6.83× |
| 10,000 | 0.036 | 274.98M | 0.034 | 292.07M | 0.061 | 1.68× | 1.79× |
| 100,000 | 0.335 | 298.64M | 0.306 | 326.45M | 0.420 | 1.25× | 1.37× |
| 1,000,000 | 3.593 | 278.30M | 3.709 | 269.58M | 3.381 | 0.94× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.102 | 1.31× |
| 1 | 5 | 0.290 | 0.465 | 1.60× |
| 1 | 10 | 0.567 | 0.997 | 1.76× |
| 10 | 1 | 0.051 | 0.086 | 1.70× |
| 10 | 5 | 0.277 | 0.437 | 1.58× |
| 10 | 10 | 0.528 | 1.120 | 2.12× |
| 100 | 1 | 0.049 | 0.089 | 1.82× |
| 100 | 5 | 0.226 | 0.472 | 2.09× |
| 100 | 10 | 0.487 | 0.941 | 1.93× |
| 1,000 | 1 | 0.062 | 0.104 | 1.67× |
| 1,000 | 5 | 0.265 | 0.511 | 1.93× |
| 1,000 | 10 | 0.521 | 0.936 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
