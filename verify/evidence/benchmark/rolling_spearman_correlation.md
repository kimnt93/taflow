# RollingSpearmanCorrelation benchmark (`SpearmanCorrelation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.447 | 2.24M | 0.419 | 2.38M | 0.802 | 1.79× | 1.91× |
| 10,000 | 4.189 | 2.39M | 4.265 | 2.34M | 6.627 | 1.58× | 1.55× |
| 100,000 | 42.456 | 2.36M | 43.286 | 2.31M | 64.175 | 1.51× | 1.48× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.259 | 3.15× |
| 1 | 5 | 0.288 | 1.236 | 4.29× |
| 1 | 10 | 0.400 | 2.339 | 5.85× |
| 10 | 1 | 0.047 | 0.217 | 4.57× |
| 10 | 5 | 0.197 | 1.264 | 6.41× |
| 10 | 10 | 0.491 | 2.458 | 5.00× |
| 100 | 1 | 0.091 | 0.274 | 3.00× |
| 100 | 5 | 0.219 | 1.885 | 8.62× |
| 100 | 10 | 0.533 | 3.120 | 5.85× |
| 1,000 | 1 | 0.502 | 0.925 | 1.84× |
| 1,000 | 5 | 0.624 | 4.608 | 7.39× |
| 1,000 | 10 | 1.205 | 9.041 | 7.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
