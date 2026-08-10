# CandleGravestoneDoji benchmark (`CDLGRAVESTONEDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.64M | 0.011 | 91.88M | 0.043 | 3.01× | 3.98× |
| 10,000 | 0.082 | 121.92M | 0.077 | 129.77M | 0.115 | 1.41× | 1.50× |
| 100,000 | 1.099 | 90.99M | 0.794 | 125.96M | 0.788 | 0.72× | 0.99× |
| 1,000,000 | 8.392 | 119.16M | 8.111 | 123.29M | 8.031 | 0.96× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.124 | 1.33× |
| 1 | 5 | 0.296 | 0.494 | 1.67× |
| 1 | 10 | 0.626 | 0.950 | 1.52× |
| 10 | 1 | 0.057 | 0.090 | 1.57× |
| 10 | 5 | 0.254 | 0.454 | 1.79× |
| 10 | 10 | 0.560 | 0.979 | 1.75× |
| 100 | 1 | 0.065 | 0.091 | 1.41× |
| 100 | 5 | 0.252 | 0.444 | 1.76× |
| 100 | 10 | 0.604 | 0.987 | 1.63× |
| 1,000 | 1 | 0.068 | 0.107 | 1.58× |
| 1,000 | 5 | 0.294 | 0.500 | 1.70× |
| 1,000 | 10 | 0.582 | 1.023 | 1.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
