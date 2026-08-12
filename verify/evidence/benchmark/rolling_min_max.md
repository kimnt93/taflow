# RollingMinMax benchmark (`MINMAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 118.28M | 0.007 | 134.74M | 0.041 | 4.81× | 5.49× |
| 10,000 | 0.080 | 125.21M | 0.068 | 147.89M | 0.113 | 1.42× | 1.68× |
| 100,000 | 0.709 | 141.04M | 0.865 | 115.66M | 0.827 | 1.17× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.124 | 1.00× |
| 1 | 5 | 0.350 | 0.524 | 1.50× |
| 1 | 10 | 0.519 | 0.991 | 1.91× |
| 10 | 1 | 0.048 | 0.093 | 1.95× |
| 10 | 5 | 0.219 | 0.451 | 2.06× |
| 10 | 10 | 0.459 | 0.959 | 2.09× |
| 100 | 1 | 0.049 | 0.109 | 2.23× |
| 100 | 5 | 0.254 | 0.482 | 1.90× |
| 100 | 10 | 0.473 | 0.972 | 2.05× |
| 1,000 | 1 | 0.057 | 0.106 | 1.85× |
| 1,000 | 5 | 0.227 | 0.504 | 2.22× |
| 1,000 | 10 | 0.495 | 1.033 | 2.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
