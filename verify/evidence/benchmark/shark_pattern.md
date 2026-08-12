# SharkPattern benchmark (`Shark` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.17M | 0.013 | 78.68M | 0.237 | 12.39× | 18.69× |
| 10,000 | 0.108 | 92.90M | 0.102 | 97.56M | 1.463 | 13.59× | 14.27× |
| 100,000 | 1.006 | 99.37M | 0.981 | 101.96M | 14.598 | 14.51× | 14.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.217 | 2.44× |
| 1 | 5 | 0.263 | 0.824 | 3.13× |
| 1 | 10 | 0.531 | 1.759 | 3.31× |
| 10 | 1 | 0.060 | 0.167 | 2.78× |
| 10 | 5 | 0.259 | 1.097 | 4.23× |
| 10 | 10 | 0.562 | 1.812 | 3.22× |
| 100 | 1 | 0.065 | 0.192 | 2.94× |
| 100 | 5 | 0.265 | 1.217 | 4.58× |
| 100 | 10 | 0.577 | 1.933 | 3.35× |
| 1,000 | 1 | 0.068 | 0.313 | 4.61× |
| 1,000 | 5 | 0.280 | 1.921 | 6.86× |
| 1,000 | 10 | 0.587 | 3.069 | 5.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
