# CandleShortLine benchmark (`CDLSHORTLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.18M | 0.010 | 96.03M | 0.036 | 2.76× | 3.43× |
| 10,000 | 0.150 | 66.69M | 0.151 | 66.35M | 0.194 | 1.29× | 1.29× |
| 100,000 | 1.577 | 63.42M | 1.538 | 65.01M | 1.768 | 1.12× | 1.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.107 | 1.55× |
| 1 | 5 | 0.309 | 0.465 | 1.50× |
| 1 | 10 | 0.424 | 0.975 | 2.30× |
| 10 | 1 | 0.048 | 0.090 | 1.87× |
| 10 | 5 | 0.197 | 0.428 | 2.17× |
| 10 | 10 | 0.387 | 0.937 | 2.42× |
| 100 | 1 | 0.047 | 0.099 | 2.09× |
| 100 | 5 | 0.198 | 0.444 | 2.24× |
| 100 | 10 | 0.400 | 0.939 | 2.34× |
| 1,000 | 1 | 0.059 | 0.110 | 1.88× |
| 1,000 | 5 | 0.213 | 0.553 | 2.60× |
| 1,000 | 10 | 0.434 | 1.114 | 2.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
