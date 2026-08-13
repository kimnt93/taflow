# RollingTreynorRatio benchmark (`TreynorRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.083 | 12.11M | 0.077 | 12.90M | 0.227 | 2.75× | 2.93× |
| 10,000 | 0.664 | 15.05M | 0.740 | 13.51M | 0.931 | 1.40× | 1.26× |
| 100,000 | 6.528 | 15.32M | 7.908 | 12.65M | 9.619 | 1.47× | 1.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.288 | 2.38× |
| 1 | 5 | 0.392 | 1.174 | 3.00× |
| 1 | 10 | 0.786 | 2.638 | 3.35× |
| 10 | 1 | 0.093 | 0.254 | 2.72× |
| 10 | 5 | 0.350 | 1.410 | 4.03× |
| 10 | 10 | 0.705 | 2.526 | 3.58× |
| 100 | 1 | 0.090 | 0.264 | 2.93× |
| 100 | 5 | 0.352 | 1.412 | 4.01× |
| 100 | 10 | 0.698 | 2.553 | 3.66× |
| 1,000 | 1 | 0.169 | 1.458 | 8.64× |
| 1,000 | 5 | 1.835 | 2.670 | 1.46× |
| 1,000 | 10 | 1.089 | 5.238 | 4.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
