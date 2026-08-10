# GarmanKlassYangZhang benchmark (`annualized Garman-Klass-Yang-Zhang volatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 32.89M | 0.027 | 36.93M | 0.118 | 3.88× | 4.36× |
| 10,000 | 0.215 | 46.53M | 0.210 | 47.62M | 0.443 | 2.06× | 2.11× |
| 100,000 | 2.010 | 49.76M | 2.080 | 48.07M | 3.690 | 1.84× | 1.77× |
| 1,000,000 | 21.042 | 47.52M | 20.510 | 48.76M | 38.236 | 1.82× | 1.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.143 | 1.56× |
| 1 | 5 | 0.366 | 0.687 | 1.87× |
| 1 | 10 | 0.538 | 1.286 | 2.39× |
| 10 | 1 | 0.059 | 0.127 | 2.13× |
| 10 | 5 | 0.251 | 0.607 | 2.42× |
| 10 | 10 | 0.527 | 1.254 | 2.38× |
| 100 | 1 | 0.056 | 0.167 | 2.99× |
| 100 | 5 | 0.273 | 0.794 | 2.92× |
| 100 | 10 | 0.556 | 1.610 | 2.90× |
| 1,000 | 1 | 0.081 | 0.199 | 2.46× |
| 1,000 | 5 | 0.270 | 1.025 | 3.80× |
| 1,000 | 10 | 0.570 | 2.243 | 3.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
