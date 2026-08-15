# RollingAverageDeviation benchmark (`AVGDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.16M | 0.018 | 56.99M | 0.049 | 2.72× | 2.81× |
| 10,000 | 0.180 | 55.52M | 0.170 | 58.78M | 0.183 | 1.01× | 1.07× |
| 100,000 | 1.804 | 55.45M | 1.631 | 61.33M | 1.488 | 0.82× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.125 | 1.32× |
| 1 | 5 | 0.244 | 0.490 | 2.01× |
| 1 | 10 | 0.422 | 0.943 | 2.24× |
| 10 | 1 | 0.049 | 0.100 | 2.04× |
| 10 | 5 | 0.199 | 0.444 | 2.22× |
| 10 | 10 | 0.390 | 0.985 | 2.53× |
| 100 | 1 | 0.044 | 0.090 | 2.05× |
| 100 | 5 | 0.191 | 0.430 | 2.25× |
| 100 | 10 | 0.386 | 0.908 | 2.35× |
| 1,000 | 1 | 0.058 | 0.105 | 1.81× |
| 1,000 | 5 | 0.209 | 0.578 | 2.76× |
| 1,000 | 10 | 0.419 | 1.095 | 2.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
