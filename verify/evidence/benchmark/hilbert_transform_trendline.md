# HilbertTransformTrendline benchmark (`HT_TRENDLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.066 | 15.19M | 0.066 | 15.21M | 0.089 | 1.36× | 1.36× |
| 10,000 | 0.688 | 14.54M | 0.742 | 13.49M | 0.603 | 0.88× | 0.81× |
| 100,000 | 6.694 | 14.94M | 6.914 | 14.46M | 5.595 | 0.84× | 0.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.054 | 0.121 | 2.25× |
| 1 | 5 | 0.232 | 0.443 | 1.91× |
| 1 | 10 | 0.387 | 0.901 | 2.33× |
| 10 | 1 | 0.046 | 0.090 | 1.96× |
| 10 | 5 | 0.181 | 0.446 | 2.47× |
| 10 | 10 | 0.413 | 0.910 | 2.20× |
| 100 | 1 | 0.051 | 0.100 | 1.94× |
| 100 | 5 | 0.213 | 0.455 | 2.14× |
| 100 | 10 | 0.475 | 0.990 | 2.08× |
| 1,000 | 1 | 0.110 | 0.158 | 1.43× |
| 1,000 | 5 | 0.259 | 0.743 | 2.87× |
| 1,000 | 10 | 0.503 | 1.572 | 3.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
