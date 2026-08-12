# VariableIndexDynamicAverage benchmark (`VIDYA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.10M | 0.014 | 70.80M | 0.231 | 15.96× | 16.36× |
| 10,000 | 0.120 | 83.11M | 0.114 | 87.66M | 0.592 | 4.92× | 5.19× |
| 100,000 | 1.241 | 80.57M | 1.117 | 89.53M | 4.256 | 3.43× | 3.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.305 | 2.65× |
| 1 | 5 | 0.331 | 1.567 | 4.74× |
| 1 | 10 | 0.513 | 3.183 | 6.20× |
| 10 | 1 | 0.054 | 0.267 | 4.99× |
| 10 | 5 | 0.277 | 1.708 | 6.17× |
| 10 | 10 | 0.495 | 2.871 | 5.80× |
| 100 | 1 | 0.057 | 0.270 | 4.74× |
| 100 | 5 | 0.257 | 1.607 | 6.24× |
| 100 | 10 | 0.531 | 3.088 | 5.81× |
| 1,000 | 1 | 0.066 | 0.303 | 4.62× |
| 1,000 | 5 | 0.247 | 1.861 | 7.54× |
| 1,000 | 10 | 0.531 | 3.274 | 6.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
