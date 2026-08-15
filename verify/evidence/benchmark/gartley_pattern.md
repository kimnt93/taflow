# GartleyPattern benchmark (`Gartley` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.10M | 0.007 | 139.21M | 0.233 | 24.03× | 32.44× |
| 10,000 | 0.094 | 106.49M | 0.089 | 112.48M | 1.357 | 14.45× | 15.26× |
| 100,000 | 0.933 | 107.24M | 0.899 | 111.22M | 12.862 | 13.79× | 14.31× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.191 | 3.05× |
| 1 | 5 | 0.253 | 0.793 | 3.14× |
| 1 | 10 | 0.368 | 1.780 | 4.84× |
| 10 | 1 | 0.042 | 0.163 | 3.89× |
| 10 | 5 | 0.203 | 1.099 | 5.40× |
| 10 | 10 | 0.384 | 1.695 | 4.41× |
| 100 | 1 | 0.050 | 0.177 | 3.55× |
| 100 | 5 | 0.197 | 1.139 | 5.78× |
| 100 | 10 | 0.410 | 1.836 | 4.48× |
| 1,000 | 1 | 0.057 | 0.302 | 5.26× |
| 1,000 | 5 | 0.215 | 1.802 | 8.39× |
| 1,000 | 10 | 0.477 | 3.067 | 6.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
