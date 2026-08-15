# CandleInNeck benchmark (`CDLINNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 150.57M | 0.003 | 335.72M | 0.034 | 5.17× | 11.53× |
| 10,000 | 0.058 | 172.06M | 0.052 | 190.55M | 0.131 | 2.26× | 2.50× |
| 100,000 | 0.966 | 103.51M | 0.818 | 122.20M | 0.954 | 0.99× | 1.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.058 | 0.107 | 1.86× |
| 1 | 5 | 0.220 | 0.453 | 2.05× |
| 1 | 10 | 0.375 | 0.892 | 2.38× |
| 10 | 1 | 0.042 | 0.099 | 2.36× |
| 10 | 5 | 0.197 | 0.451 | 2.29× |
| 10 | 10 | 0.374 | 0.894 | 2.39× |
| 100 | 1 | 0.040 | 0.089 | 2.22× |
| 100 | 5 | 0.188 | 0.452 | 2.40× |
| 100 | 10 | 0.646 | 0.876 | 1.35× |
| 1,000 | 1 | 0.047 | 0.099 | 2.13× |
| 1,000 | 5 | 0.205 | 0.509 | 2.48× |
| 1,000 | 10 | 0.425 | 1.021 | 2.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
