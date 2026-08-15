# RollingRecoveryFactor benchmark (`rolling recovery factor on equity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.83M | 0.050 | 20.02M | 0.208 | 4.33× | 4.16× |
| 10,000 | 0.466 | 21.46M | 0.459 | 21.78M | 1.331 | 2.86× | 2.90× |
| 100,000 | 4.660 | 21.46M | 4.574 | 21.86M | 16.986 | 3.65× | 3.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.129 | 1.21× |
| 1 | 5 | 0.287 | 0.452 | 1.57× |
| 1 | 10 | 0.401 | 1.064 | 2.65× |
| 10 | 1 | 0.044 | 0.094 | 2.15× |
| 10 | 5 | 0.224 | 0.474 | 2.11× |
| 10 | 10 | 0.379 | 0.889 | 2.34× |
| 100 | 1 | 0.051 | 0.195 | 3.81× |
| 100 | 5 | 0.201 | 1.018 | 5.06× |
| 100 | 10 | 0.466 | 1.944 | 4.17× |
| 1,000 | 1 | 0.099 | 0.293 | 2.94× |
| 1,000 | 5 | 0.249 | 1.219 | 4.90× |
| 1,000 | 10 | 0.467 | 2.337 | 5.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
