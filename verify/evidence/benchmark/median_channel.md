# MedianChannel benchmark (`MedianChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 3.143 | 318.18K | 3.160 | 316.46K | 0.942 | 0.30× | 0.30× |
| 10,000 | 31.750 | 314.96K | 31.783 | 314.64K | 7.578 | 0.24× | 0.24× |
| 100,000 | 317.877 | 314.59K | 323.281 | 309.33K | 78.773 | 0.25× | 0.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.279 | 2.41× |
| 1 | 5 | 0.377 | 1.378 | 3.65× |
| 1 | 10 | 0.631 | 2.612 | 4.14× |
| 10 | 1 | 0.077 | 0.254 | 3.31× |
| 10 | 5 | 0.317 | 1.412 | 4.46× |
| 10 | 10 | 0.624 | 2.737 | 4.38× |
| 100 | 1 | 0.354 | 0.329 | 0.93× |
| 100 | 5 | 0.551 | 1.795 | 3.26× |
| 100 | 10 | 0.940 | 3.347 | 3.56× |
| 1,000 | 1 | 3.265 | 1.217 | 0.37× |
| 1,000 | 5 | 3.635 | 5.576 | 1.53× |
| 1,000 | 10 | 6.322 | 11.401 | 1.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
