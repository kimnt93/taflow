# CandleUpsideGapTwoCrows benchmark (`CDLUPSIDEGAP2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.71M | 0.007 | 133.54M | 0.032 | 3.43× | 4.25× |
| 10,000 | 0.087 | 114.44M | 0.084 | 118.88M | 0.122 | 1.39× | 1.45× |
| 100,000 | 0.874 | 114.39M | 0.873 | 114.53M | 1.011 | 1.16× | 1.16× |
| 1,000,000 | 9.204 | 108.65M | 9.402 | 106.36M | 9.669 | 1.05× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.126 | 1.13× |
| 1 | 5 | 0.297 | 0.465 | 1.57× |
| 1 | 10 | 0.546 | 0.963 | 1.77× |
| 10 | 1 | 0.053 | 0.097 | 1.84× |
| 10 | 5 | 0.255 | 0.461 | 1.81× |
| 10 | 10 | 0.523 | 0.978 | 1.87× |
| 100 | 1 | 0.053 | 0.092 | 1.72× |
| 100 | 5 | 0.267 | 0.456 | 1.71× |
| 100 | 10 | 0.554 | 0.954 | 1.72× |
| 1,000 | 1 | 0.072 | 0.102 | 1.42× |
| 1,000 | 5 | 0.269 | 0.512 | 1.91× |
| 1,000 | 10 | 0.579 | 1.099 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
