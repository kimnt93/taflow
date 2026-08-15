# YangZhang benchmark (`YangZhangVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.70M | 0.046 | 21.53M | 0.351 | 6.91× | 7.56× |
| 10,000 | 0.428 | 23.36M | 0.423 | 23.66M | 1.892 | 4.42× | 4.48× |
| 100,000 | 4.384 | 22.81M | 4.327 | 23.11M | 17.436 | 3.98× | 4.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.270 | 2.94× |
| 1 | 5 | 0.246 | 1.487 | 6.03× |
| 1 | 10 | 0.396 | 2.689 | 6.80× |
| 10 | 1 | 0.072 | 0.272 | 3.78× |
| 10 | 5 | 0.192 | 1.466 | 7.64× |
| 10 | 10 | 0.408 | 2.913 | 7.14× |
| 100 | 1 | 0.054 | 0.258 | 4.73× |
| 100 | 5 | 0.208 | 1.530 | 7.36× |
| 100 | 10 | 0.515 | 2.990 | 5.81× |
| 1,000 | 1 | 0.090 | 0.417 | 4.65× |
| 1,000 | 5 | 0.247 | 2.520 | 10.22× |
| 1,000 | 10 | 0.456 | 4.707 | 10.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
