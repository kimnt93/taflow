# CandleTakuri benchmark (`CDLTAKURI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.31M | 0.003 | 332.87M | 0.037 | 6.06× | 12.27× |
| 10,000 | 0.045 | 222.34M | 0.040 | 247.02M | 0.110 | 2.44× | 2.71× |
| 100,000 | 0.538 | 186.00M | 0.520 | 192.46M | 0.809 | 1.50× | 1.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | 0.120 | 1.80× |
| 1 | 5 | 0.241 | 0.449 | 1.86× |
| 1 | 10 | 0.382 | 0.926 | 2.43× |
| 10 | 1 | 0.042 | 0.089 | 2.11× |
| 10 | 5 | 0.198 | 0.456 | 2.30× |
| 10 | 10 | 0.371 | 0.913 | 2.46× |
| 100 | 1 | 0.041 | 0.091 | 2.20× |
| 100 | 5 | 0.197 | 0.417 | 2.12× |
| 100 | 10 | 0.424 | 0.944 | 2.23× |
| 1,000 | 1 | 0.048 | 0.108 | 2.24× |
| 1,000 | 5 | 0.199 | 0.478 | 2.41× |
| 1,000 | 10 | 0.388 | 1.031 | 2.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
