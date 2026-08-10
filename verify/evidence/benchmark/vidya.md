# VariableIndexDynamicAverage benchmark (`VIDYA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.29M | 0.013 | 74.22M | 0.202 | 14.62× | 15.01× |
| 10,000 | 0.118 | 84.82M | 0.116 | 86.09M | 0.582 | 4.94× | 5.01× |
| 100,000 | 1.168 | 85.65M | 1.135 | 88.08M | 4.211 | 3.61× | 3.71× |
| 1,000,000 | 11.726 | 85.28M | 11.115 | 89.97M | 39.388 | 3.36× | 3.54× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.331 | 5.06× |
| 1 | 5 | 0.312 | 1.304 | 4.18× |
| 1 | 10 | 0.505 | 2.740 | 5.42× |
| 10 | 1 | 0.048 | 0.262 | 5.41× |
| 10 | 5 | 0.224 | 1.503 | 6.72× |
| 10 | 10 | 0.501 | 2.896 | 5.78× |
| 100 | 1 | 0.059 | 0.250 | 4.27× |
| 100 | 5 | 0.242 | 1.534 | 6.34× |
| 100 | 10 | 0.471 | 2.691 | 5.71× |
| 1,000 | 1 | 0.058 | 0.291 | 5.01× |
| 1,000 | 5 | 0.248 | 1.731 | 6.99× |
| 1,000 | 10 | 0.513 | 3.344 | 6.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
