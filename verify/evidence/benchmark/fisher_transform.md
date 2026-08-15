# FisherTransform benchmark (`fisher` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 28.00M | 0.034 | 29.10M | 1.227 | 34.35× | 35.70× |
| 10,000 | 0.374 | 26.77M | 0.371 | 26.92M | 1.652 | 4.42× | 4.45× |
| 100,000 | 3.716 | 26.91M | 3.709 | 26.96M | 6.442 | 1.73× | 1.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.236 | 2.00× |
| 1 | 5 | 0.335 | 0.947 | 2.83× |
| 1 | 10 | 0.414 | 1.777 | 4.30× |
| 10 | 1 | 0.048 | 1.345 | 27.88× |
| 10 | 5 | 0.186 | 6.771 | 36.34× |
| 10 | 10 | 0.407 | 13.049 | 32.09× |
| 100 | 1 | 0.048 | 1.231 | 25.44× |
| 100 | 5 | 0.198 | 6.365 | 32.16× |
| 100 | 10 | 0.441 | 12.962 | 29.42× |
| 1,000 | 1 | 0.092 | 1.350 | 14.72× |
| 1,000 | 5 | 0.208 | 7.244 | 34.85× |
| 1,000 | 10 | 0.497 | 14.316 | 28.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
