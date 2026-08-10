# RollingWinsorize benchmark (`rolling winsorize` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.059 | 16.91M | 0.058 | 17.32M | 0.598 | 10.11× | 10.35× |
| 10,000 | 0.563 | 17.76M | 0.564 | 17.73M | 3.108 | 5.52× | 5.51× |
| 100,000 | 5.496 | 18.19M | 7.044 | 14.20M | 32.300 | 5.88× | 4.59× |
| 1,000,000 | 55.582 | 17.99M | 54.033 | 18.51M | 323.462 | 5.82× | 5.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.367 | 4.70× |
| 1 | 5 | 0.342 | 1.645 | 4.81× |
| 1 | 10 | 0.484 | 3.337 | 6.89× |
| 10 | 1 | 0.053 | 0.304 | 5.70× |
| 10 | 5 | 0.228 | 1.576 | 6.91× |
| 10 | 10 | 0.472 | 3.313 | 7.01× |
| 100 | 1 | 0.058 | 0.376 | 6.48× |
| 100 | 5 | 0.242 | 1.973 | 8.15× |
| 100 | 10 | 0.513 | 3.988 | 7.77× |
| 1,000 | 1 | 0.113 | 0.652 | 5.79× |
| 1,000 | 5 | 0.263 | 2.289 | 8.70× |
| 1,000 | 10 | 0.546 | 4.917 | 9.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
