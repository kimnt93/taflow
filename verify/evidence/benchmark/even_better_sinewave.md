# EvenBetterSinewave benchmark (`ebsw` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 26.44M | 0.033 | 30.71M | 11.488 | 303.77× | 352.78× |
| 10,000 | 0.266 | 37.53M | 0.256 | 39.04M | 116.290 | 436.48× | 454.05× |
| 100,000 | 2.486 | 40.23M | 2.530 | 39.52M | 1172.135 | 471.55× | 463.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.164 | 0.224 | 1.36× |
| 1 | 5 | 0.375 | 0.735 | 1.96× |
| 1 | 10 | 0.605 | 1.468 | 2.42× |
| 10 | 1 | 0.065 | 0.150 | 2.30× |
| 10 | 5 | 0.284 | 0.705 | 2.48× |
| 10 | 10 | 0.622 | 1.456 | 2.34× |
| 100 | 1 | 0.072 | 0.983 | 13.64× |
| 100 | 5 | 0.297 | 4.831 | 16.25× |
| 100 | 10 | 0.576 | 9.598 | 16.67× |
| 1,000 | 1 | 0.104 | 11.406 | 110.05× |
| 1,000 | 5 | 0.456 | 114.726 | 251.74× |
| 1,000 | 10 | 1.540 | 174.860 | 113.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
