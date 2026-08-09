# CandleTriStar benchmark (`CDLTRISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 112.96M | 0.007 | 147.33M | 0.035 | 3.98× | 5.19× |
| 10,000 | 0.049 | 203.91M | 0.047 | 211.23M | 0.093 | 1.89× | 1.96× |
| 100,000 | 0.552 | 181.07M | 0.598 | 167.32M | 0.642 | 1.16× | 1.07× |
| 1,000,000 | 5.942 | 168.29M | 5.849 | 170.97M | 5.989 | 1.01× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.129 | 1.17× |
| 1 | 5 | 0.325 | 0.498 | 1.53× |
| 1 | 10 | 0.506 | 0.958 | 1.89× |
| 10 | 1 | 0.054 | 0.099 | 1.82× |
| 10 | 5 | 0.240 | 0.431 | 1.80× |
| 10 | 10 | 0.523 | 0.964 | 1.84× |
| 100 | 1 | 0.052 | 0.093 | 1.78× |
| 100 | 5 | 0.274 | 0.457 | 1.67× |
| 100 | 10 | 0.509 | 0.955 | 1.88× |
| 1,000 | 1 | 0.062 | 0.100 | 1.62× |
| 1,000 | 5 | 0.244 | 0.455 | 1.86× |
| 1,000 | 10 | 0.523 | 0.998 | 1.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
