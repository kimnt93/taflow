# CandleThreeLineStrike benchmark (`CDL3LINESTRIKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.56M | 0.009 | 109.03M | 0.031 | 2.68× | 3.41× |
| 10,000 | 0.065 | 153.68M | 0.059 | 168.80M | 0.106 | 1.64× | 1.80× |
| 100,000 | 0.787 | 127.09M | 0.814 | 122.81M | 0.767 | 0.98× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.112 | 1.62× |
| 1 | 5 | 0.341 | 0.439 | 1.29× |
| 1 | 10 | 0.500 | 0.881 | 1.76× |
| 10 | 1 | 0.053 | 0.089 | 1.70× |
| 10 | 5 | 0.248 | 0.425 | 1.71× |
| 10 | 10 | 0.535 | 0.883 | 1.65× |
| 100 | 1 | 0.055 | 0.086 | 1.58× |
| 100 | 5 | 0.263 | 0.428 | 1.63× |
| 100 | 10 | 0.526 | 0.908 | 1.73× |
| 1,000 | 1 | 0.066 | 0.101 | 1.52× |
| 1,000 | 5 | 0.259 | 0.467 | 1.80× |
| 1,000 | 10 | 0.563 | 0.979 | 1.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
