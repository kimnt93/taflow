# RollingMaximum benchmark (`MAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 183.87M | 0.005 | 212.44M | 0.037 | 6.74× | 7.79× |
| 10,000 | 0.034 | 293.39M | 0.033 | 306.65M | 0.076 | 2.22× | 2.32× |
| 100,000 | 0.343 | 291.68M | 0.319 | 313.42M | 0.539 | 1.57× | 1.69× |
| 1,000,000 | 3.991 | 250.58M | 3.588 | 278.71M | 4.776 | 1.20× | 1.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.140 | 0.117 | 0.84× |
| 1 | 5 | 0.361 | 0.473 | 1.31× |
| 1 | 10 | 0.455 | 0.922 | 2.03× |
| 10 | 1 | 0.047 | 0.092 | 1.93× |
| 10 | 5 | 0.226 | 0.458 | 2.03× |
| 10 | 10 | 0.471 | 0.968 | 2.05× |
| 100 | 1 | 0.051 | 0.089 | 1.74× |
| 100 | 5 | 0.230 | 0.466 | 2.02× |
| 100 | 10 | 0.461 | 0.921 | 2.00× |
| 1,000 | 1 | 0.057 | 0.103 | 1.83× |
| 1,000 | 5 | 0.215 | 0.455 | 2.11× |
| 1,000 | 10 | 0.504 | 0.960 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
