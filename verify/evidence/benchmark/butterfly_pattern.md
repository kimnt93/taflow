# ButterflyPattern benchmark (`Butterfly` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 95.10M | 0.008 | 126.59M | 0.229 | 21.79× | 29.01× |
| 10,000 | 0.106 | 94.55M | 0.092 | 108.91M | 1.380 | 13.05× | 15.03× |
| 100,000 | 0.953 | 104.91M | 0.921 | 108.57M | 13.320 | 13.97× | 14.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.225 | 2.90× |
| 1 | 5 | 0.272 | 0.799 | 2.94× |
| 1 | 10 | 0.433 | 1.654 | 3.82× |
| 10 | 1 | 0.046 | 0.168 | 3.65× |
| 10 | 5 | 0.193 | 1.140 | 5.91× |
| 10 | 10 | 0.419 | 1.675 | 4.00× |
| 100 | 1 | 0.051 | 0.175 | 3.45× |
| 100 | 5 | 0.211 | 1.215 | 5.75× |
| 100 | 10 | 0.440 | 1.815 | 4.13× |
| 1,000 | 1 | 0.066 | 0.293 | 4.45× |
| 1,000 | 5 | 0.200 | 1.787 | 8.92× |
| 1,000 | 10 | 0.425 | 3.014 | 7.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
