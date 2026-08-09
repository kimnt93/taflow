# CandleThreeStarsInSouth benchmark (`CDL3STARSINSOUTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.42M | 0.008 | 130.92M | 0.033 | 3.58× | 4.36× |
| 10,000 | 0.062 | 162.27M | 0.059 | 168.57M | 0.112 | 1.81× | 1.88× |
| 100,000 | 0.662 | 151.09M | 0.644 | 155.19M | 0.842 | 1.27× | 1.31× |
| 1,000,000 | 6.954 | 143.80M | 6.814 | 146.75M | 8.478 | 1.22× | 1.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.122 | 1.78× |
| 1 | 5 | 0.292 | 0.524 | 1.80× |
| 1 | 10 | 0.572 | 1.033 | 1.81× |
| 10 | 1 | 0.055 | 0.090 | 1.65× |
| 10 | 5 | 0.279 | 0.485 | 1.74× |
| 10 | 10 | 0.573 | 1.011 | 1.77× |
| 100 | 1 | 0.064 | 0.100 | 1.56× |
| 100 | 5 | 0.252 | 0.464 | 1.84× |
| 100 | 10 | 0.529 | 0.912 | 1.72× |
| 1,000 | 1 | 0.062 | 0.103 | 1.67× |
| 1,000 | 5 | 0.270 | 0.487 | 1.80× |
| 1,000 | 10 | 0.538 | 1.047 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
