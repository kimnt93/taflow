# CandleTwoCrows benchmark (`CDL2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 82.93M | 0.009 | 115.50M | 0.032 | 2.64× | 3.67× |
| 10,000 | 0.075 | 132.67M | 0.069 | 144.23M | 0.112 | 1.49× | 1.62× |
| 100,000 | 0.982 | 101.87M | 0.937 | 106.76M | 0.937 | 0.95× | 1.00× |
| 1,000,000 | 10.080 | 99.21M | 10.027 | 99.74M | 9.190 | 0.91× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.125 | 1.42× |
| 1 | 5 | 0.328 | 0.542 | 1.65× |
| 1 | 10 | 0.610 | 0.986 | 1.62× |
| 10 | 1 | 0.063 | 0.095 | 1.51× |
| 10 | 5 | 0.263 | 0.520 | 1.98× |
| 10 | 10 | 0.622 | 1.062 | 1.71× |
| 100 | 1 | 0.072 | 0.100 | 1.39× |
| 100 | 5 | 0.278 | 0.435 | 1.57× |
| 100 | 10 | 0.547 | 0.990 | 1.81× |
| 1,000 | 1 | 0.093 | 0.128 | 1.38× |
| 1,000 | 5 | 0.279 | 0.522 | 1.87× |
| 1,000 | 10 | 0.575 | 0.985 | 1.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
