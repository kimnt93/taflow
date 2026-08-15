# LogReturn benchmark (`LogReturn` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 113.31M | 0.008 | 123.88M | 0.209 | 23.66× | 25.87× |
| 10,000 | 0.076 | 130.90M | 0.074 | 134.58M | 0.570 | 7.46× | 7.67× |
| 100,000 | 0.735 | 136.14M | 0.731 | 136.87M | 5.341 | 7.27× | 7.31× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.244 | 3.13× |
| 1 | 5 | 0.265 | 1.115 | 4.21× |
| 1 | 10 | 0.438 | 2.397 | 5.48× |
| 10 | 1 | 0.049 | 0.238 | 4.82× |
| 10 | 5 | 0.198 | 1.538 | 7.78× |
| 10 | 10 | 0.457 | 2.473 | 5.41× |
| 100 | 1 | 0.051 | 0.248 | 4.89× |
| 100 | 5 | 0.217 | 1.390 | 6.41× |
| 100 | 10 | 0.434 | 9.059 | 20.88× |
| 1,000 | 1 | 0.052 | 0.260 | 4.95× |
| 1,000 | 5 | 0.205 | 1.367 | 6.66× |
| 1,000 | 10 | 0.452 | 2.683 | 5.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
