# RollingStandardDeviation benchmark (`STDDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 134.66M | 0.006 | 164.51M | 0.035 | 4.67× | 5.71× |
| 10,000 | 0.045 | 220.81M | 0.040 | 250.62M | 0.058 | 1.29× | 1.46× |
| 100,000 | 0.429 | 233.37M | 0.426 | 234.71M | 0.297 | 0.69× | 0.70× |
| 1,000,000 | 4.662 | 214.48M | 3.765 | 265.63M | 3.437 | 0.74× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.154 | 0.129 | 0.84× |
| 1 | 5 | 0.346 | 0.461 | 1.33× |
| 1 | 10 | 0.514 | 1.007 | 1.96× |
| 10 | 1 | 0.048 | 0.094 | 1.96× |
| 10 | 5 | 0.224 | 0.459 | 2.04× |
| 10 | 10 | 0.458 | 0.968 | 2.11× |
| 100 | 1 | 0.049 | 0.096 | 1.96× |
| 100 | 5 | 0.231 | 0.472 | 2.04× |
| 100 | 10 | 0.460 | 0.971 | 2.11× |
| 1,000 | 1 | 0.055 | 0.097 | 1.78× |
| 1,000 | 5 | 0.216 | 0.490 | 2.27× |
| 1,000 | 10 | 0.486 | 1.008 | 2.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
