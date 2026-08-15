# MassIndex benchmark (`MassIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 121.17M | 0.007 | 136.94M | 0.225 | 27.27× | 30.82× |
| 10,000 | 0.057 | 175.34M | 0.053 | 190.17M | 0.808 | 14.17× | 15.37× |
| 100,000 | 0.618 | 161.91M | 0.513 | 195.04M | 6.559 | 10.62× | 12.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.270 | 3.75× |
| 1 | 5 | 0.367 | 1.468 | 4.00× |
| 1 | 10 | 0.405 | 2.715 | 6.70× |
| 10 | 1 | 0.046 | 0.233 | 5.10× |
| 10 | 5 | 0.193 | 1.514 | 7.85× |
| 10 | 10 | 0.381 | 2.541 | 6.67× |
| 100 | 1 | 0.047 | 0.253 | 5.39× |
| 100 | 5 | 0.202 | 1.485 | 7.36× |
| 100 | 10 | 0.404 | 2.806 | 6.95× |
| 1,000 | 1 | 0.054 | 0.316 | 5.85× |
| 1,000 | 5 | 0.212 | 1.883 | 8.86× |
| 1,000 | 10 | 0.418 | 3.185 | 7.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
