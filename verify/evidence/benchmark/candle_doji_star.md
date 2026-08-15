# CandleDojiStar benchmark (`CDLDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.27M | 0.010 | 95.37M | 0.037 | 2.76× | 3.49× |
| 10,000 | 0.151 | 66.28M | 0.144 | 69.46M | 0.133 | 0.88× | 0.92× |
| 100,000 | 1.675 | 59.72M | 1.563 | 63.97M | 1.043 | 0.62× | 0.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.103 | 1.39× |
| 1 | 5 | 0.303 | 0.489 | 1.62× |
| 1 | 10 | 0.415 | 0.931 | 2.24× |
| 10 | 1 | 0.045 | 0.087 | 1.93× |
| 10 | 5 | 0.190 | 0.419 | 2.20× |
| 10 | 10 | 0.397 | 0.933 | 2.35× |
| 100 | 1 | 0.048 | 0.090 | 1.88× |
| 100 | 5 | 0.186 | 0.415 | 2.23× |
| 100 | 10 | 0.413 | 0.875 | 2.12× |
| 1,000 | 1 | 0.058 | 0.119 | 2.03× |
| 1,000 | 5 | 0.228 | 0.507 | 2.22× |
| 1,000 | 10 | 0.436 | 0.988 | 2.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
