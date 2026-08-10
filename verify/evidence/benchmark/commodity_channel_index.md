# CommodityChannelIndex benchmark (`CCI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 38.56M | 0.025 | 40.71M | 0.054 | 2.08× | 2.19× |
| 10,000 | 0.208 | 48.09M | 0.207 | 48.22M | 0.243 | 1.17× | 1.17× |
| 100,000 | 2.103 | 47.56M | 2.759 | 36.25M | 2.224 | 1.06× | 0.81× |
| 1,000,000 | 22.044 | 45.36M | 21.190 | 47.19M | 21.840 | 0.99× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.136 | 0.128 | 0.94× |
| 1 | 5 | 0.316 | 0.500 | 1.58× |
| 1 | 10 | 0.479 | 0.952 | 1.99× |
| 10 | 1 | 0.052 | 0.092 | 1.78× |
| 10 | 5 | 0.230 | 0.442 | 1.93× |
| 10 | 10 | 0.490 | 0.975 | 1.99× |
| 100 | 1 | 0.059 | 0.102 | 1.73× |
| 100 | 5 | 0.259 | 0.489 | 1.89× |
| 100 | 10 | 0.504 | 0.945 | 1.87× |
| 1,000 | 1 | 0.073 | 0.118 | 1.60× |
| 1,000 | 5 | 0.265 | 0.602 | 2.27× |
| 1,000 | 10 | 0.578 | 1.221 | 2.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
