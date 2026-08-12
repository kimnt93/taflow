# DoubleExponentialMovingAverage benchmark (`DEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.62M | 0.009 | 110.10M | 0.037 | 3.58× | 4.12× |
| 10,000 | 0.065 | 153.02M | 0.064 | 155.90M | 0.090 | 1.38× | 1.41× |
| 100,000 | 0.652 | 153.33M | 0.656 | 152.38M | 1.086 | 1.66× | 1.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.175 | 2.70× |
| 1 | 5 | 0.285 | 0.526 | 1.84× |
| 1 | 10 | 0.512 | 0.952 | 1.86× |
| 10 | 1 | 0.057 | 0.099 | 1.73× |
| 10 | 5 | 0.268 | 0.513 | 1.91× |
| 10 | 10 | 0.464 | 0.947 | 2.04× |
| 100 | 1 | 0.052 | 0.097 | 1.88× |
| 100 | 5 | 0.227 | 0.537 | 2.37× |
| 100 | 10 | 0.521 | 0.987 | 1.89× |
| 1,000 | 1 | 0.061 | 0.097 | 1.58× |
| 1,000 | 5 | 0.242 | 0.468 | 1.93× |
| 1,000 | 10 | 0.536 | 1.078 | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
