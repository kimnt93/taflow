# CandleAdvanceBlock benchmark (`CDLADVANCEBLOCK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.68M | 0.007 | 148.16M | 0.048 | 4.93× | 7.11× |
| 10,000 | 0.086 | 116.81M | 0.082 | 121.60M | 0.224 | 2.62× | 2.73× |
| 100,000 | 0.934 | 107.12M | 0.887 | 112.68M | 1.996 | 2.14× | 2.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.129 | 1.22× |
| 1 | 5 | 0.263 | 0.440 | 1.68× |
| 1 | 10 | 0.396 | 0.881 | 2.22× |
| 10 | 1 | 0.042 | 0.090 | 2.13× |
| 10 | 5 | 0.183 | 0.445 | 2.43× |
| 10 | 10 | 0.437 | 0.908 | 2.08× |
| 100 | 1 | 0.040 | 0.093 | 2.34× |
| 100 | 5 | 0.183 | 0.427 | 2.33× |
| 100 | 10 | 0.374 | 0.909 | 2.43× |
| 1,000 | 1 | 0.049 | 0.111 | 2.27× |
| 1,000 | 5 | 0.198 | 0.538 | 2.71× |
| 1,000 | 10 | 0.427 | 1.105 | 2.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
