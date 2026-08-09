# RollingMinMax benchmark (`MINMAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 111.76M | 0.008 | 127.25M | 0.045 | 5.03× | 5.72× |
| 10,000 | 0.075 | 132.83M | 0.072 | 137.98M | 0.128 | 1.70× | 1.76× |
| 100,000 | 0.820 | 121.99M | 0.721 | 138.75M | 0.897 | 1.09× | 1.24× |
| 1,000,000 | 9.105 | 109.83M | 8.165 | 122.48M | 8.358 | 0.92× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.120 | 1.31× |
| 1 | 5 | 0.340 | 0.608 | 1.79× |
| 1 | 10 | 0.492 | 1.035 | 2.11× |
| 10 | 1 | 0.050 | 0.101 | 2.02× |
| 10 | 5 | 0.224 | 0.484 | 2.16× |
| 10 | 10 | 0.530 | 1.040 | 1.96× |
| 100 | 1 | 0.053 | 0.102 | 1.92× |
| 100 | 5 | 0.241 | 0.478 | 1.98× |
| 100 | 10 | 0.531 | 1.139 | 2.14× |
| 1,000 | 1 | 0.067 | 0.123 | 1.83× |
| 1,000 | 5 | 0.267 | 0.543 | 2.03× |
| 1,000 | 10 | 0.551 | 1.199 | 2.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
