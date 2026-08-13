# KnowSureThing benchmark (`KST` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.123 | 8.10M | 0.117 | 8.54M | 0.694 | 5.62× | 5.93× |
| 10,000 | 1.062 | 9.42M | 1.008 | 9.92M | 3.889 | 3.66× | 3.86× |
| 100,000 | 10.297 | 9.71M | 10.104 | 9.90M | 36.052 | 3.50× | 3.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.135 | 0.526 | 3.90× |
| 1 | 5 | 0.515 | 2.587 | 5.02× |
| 1 | 10 | 0.709 | 4.989 | 7.04× |
| 10 | 1 | 0.081 | 0.479 | 5.93× |
| 10 | 5 | 0.314 | 2.588 | 8.23× |
| 10 | 10 | 0.682 | 4.995 | 7.32× |
| 100 | 1 | 0.094 | 0.497 | 5.31× |
| 100 | 5 | 0.333 | 2.732 | 8.20× |
| 100 | 10 | 0.708 | 5.449 | 7.70× |
| 1,000 | 1 | 0.183 | 1.009 | 5.50× |
| 1,000 | 5 | 0.440 | 4.603 | 10.46× |
| 1,000 | 10 | 0.787 | 9.092 | 11.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
