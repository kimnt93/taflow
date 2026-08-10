# RollingRank benchmark (`rolling percentile rank` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.36M | 0.021 | 46.68M | 0.153 | 6.62× | 7.12× |
| 10,000 | 0.177 | 56.36M | 0.177 | 56.42M | 0.749 | 4.22× | 4.23× |
| 100,000 | 1.826 | 54.77M | 1.780 | 56.17M | 6.994 | 3.83× | 3.93× |
| 1,000,000 | 18.264 | 54.75M | 18.086 | 55.29M | 73.293 | 4.01× | 4.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.158 | 0.294 | 1.87× |
| 1 | 5 | 0.256 | 0.528 | 2.06× |
| 1 | 10 | 0.523 | 1.194 | 2.28× |
| 10 | 1 | 0.060 | 0.124 | 2.07× |
| 10 | 5 | 0.229 | 0.509 | 2.22× |
| 10 | 10 | 0.495 | 1.226 | 2.48× |
| 100 | 1 | 0.063 | 0.174 | 2.74× |
| 100 | 5 | 0.250 | 0.817 | 3.26× |
| 100 | 10 | 0.564 | 1.922 | 3.41× |
| 1,000 | 1 | 0.070 | 0.261 | 3.71× |
| 1,000 | 5 | 0.285 | 1.088 | 3.82× |
| 1,000 | 10 | 0.544 | 2.359 | 4.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
