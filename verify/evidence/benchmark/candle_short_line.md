# CandleShortLine benchmark (`CDLSHORTLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.24M | 0.010 | 100.11M | 0.039 | 3.01× | 3.95× |
| 10,000 | 0.159 | 62.71M | 0.150 | 66.87M | 0.219 | 1.37× | 1.46× |
| 100,000 | 1.603 | 62.40M | 1.573 | 63.56M | 1.916 | 1.20× | 1.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.056 | 0.103 | 1.83× |
| 1 | 5 | 0.260 | 0.540 | 2.08× |
| 1 | 10 | 0.425 | 0.993 | 2.34× |
| 10 | 1 | 0.045 | 0.086 | 1.93× |
| 10 | 5 | 0.190 | 0.436 | 2.30× |
| 10 | 10 | 0.441 | 0.940 | 2.13× |
| 100 | 1 | 0.042 | 0.102 | 2.45× |
| 100 | 5 | 0.203 | 0.499 | 2.45× |
| 100 | 10 | 0.439 | 0.942 | 2.15× |
| 1,000 | 1 | 0.057 | 0.106 | 1.86× |
| 1,000 | 5 | 0.236 | 0.542 | 2.29× |
| 1,000 | 10 | 0.455 | 1.128 | 2.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
