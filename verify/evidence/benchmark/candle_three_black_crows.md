# CandleThreeBlackCrows benchmark (`CDL3BLACKCROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 82.84M | 0.008 | 118.48M | 0.031 | 2.53× | 3.62× |
| 10,000 | 0.060 | 166.06M | 0.053 | 187.80M | 0.087 | 1.45× | 1.64× |
| 100,000 | 0.765 | 130.75M | 0.684 | 146.30M | 0.606 | 0.79× | 0.89× |
| 1,000,000 | 7.280 | 137.36M | 7.394 | 135.24M | 5.889 | 0.81× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.134 | 0.150 | 1.12× |
| 1 | 5 | 0.374 | 0.438 | 1.17× |
| 1 | 10 | 0.519 | 0.891 | 1.72× |
| 10 | 1 | 0.055 | 0.093 | 1.69× |
| 10 | 5 | 0.251 | 0.421 | 1.68× |
| 10 | 10 | 0.523 | 0.886 | 1.69× |
| 100 | 1 | 0.056 | 0.085 | 1.53× |
| 100 | 5 | 0.250 | 0.419 | 1.67× |
| 100 | 10 | 0.505 | 0.881 | 1.75× |
| 1,000 | 1 | 0.060 | 0.102 | 1.71× |
| 1,000 | 5 | 0.256 | 0.451 | 1.76× |
| 1,000 | 10 | 0.593 | 1.052 | 1.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
