# RollingSum benchmark (`SUM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 148.69M | 0.005 | 197.70M | 0.030 | 4.51× | 6.00× |
| 10,000 | 0.035 | 288.91M | 0.032 | 315.49M | 0.048 | 1.40× | 1.53× |
| 100,000 | 0.318 | 314.12M | 0.303 | 330.55M | 0.203 | 0.64× | 0.67× |
| 1,000,000 | 3.974 | 251.63M | 3.095 | 323.05M | 1.866 | 0.47× | 0.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.119 | 1.26× |
| 1 | 5 | 0.327 | 0.476 | 1.46× |
| 1 | 10 | 0.472 | 0.925 | 1.96× |
| 10 | 1 | 0.051 | 0.093 | 1.84× |
| 10 | 5 | 0.220 | 0.419 | 1.90× |
| 10 | 10 | 0.450 | 0.917 | 2.04× |
| 100 | 1 | 0.047 | 0.088 | 1.88× |
| 100 | 5 | 0.218 | 0.427 | 1.96× |
| 100 | 10 | 0.445 | 0.905 | 2.03× |
| 1,000 | 1 | 0.049 | 0.088 | 1.80× |
| 1,000 | 5 | 0.216 | 0.445 | 2.06× |
| 1,000 | 10 | 0.486 | 0.931 | 1.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
