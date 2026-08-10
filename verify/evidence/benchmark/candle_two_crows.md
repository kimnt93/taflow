# CandleTwoCrows benchmark (`CDL2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 91.41M | 0.008 | 126.45M | 0.030 | 2.73× | 3.77× |
| 10,000 | 0.067 | 148.46M | 0.063 | 158.89M | 0.107 | 1.58× | 1.70× |
| 100,000 | 0.880 | 113.64M | 0.844 | 118.48M | 0.858 | 0.98× | 1.02× |
| 1,000,000 | 8.672 | 115.32M | 8.601 | 116.26M | 8.435 | 0.97× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.160 | 0.161 | 1.01× |
| 1 | 5 | 0.355 | 0.484 | 1.36× |
| 1 | 10 | 0.571 | 1.015 | 1.78× |
| 10 | 1 | 0.062 | 0.101 | 1.63× |
| 10 | 5 | 0.285 | 0.447 | 1.57× |
| 10 | 10 | 0.542 | 0.940 | 1.73× |
| 100 | 1 | 0.061 | 0.088 | 1.46× |
| 100 | 5 | 0.261 | 0.429 | 1.64× |
| 100 | 10 | 0.526 | 0.894 | 1.70× |
| 1,000 | 1 | 0.061 | 0.101 | 1.64× |
| 1,000 | 5 | 0.257 | 0.469 | 1.83× |
| 1,000 | 10 | 0.559 | 0.963 | 1.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
