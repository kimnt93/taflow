# RollingMidpoint benchmark (`MIDPOINT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 121.33M | 0.007 | 134.02M | 0.036 | 4.34× | 4.80× |
| 10,000 | 0.082 | 121.45M | 0.080 | 125.24M | 0.099 | 1.20× | 1.24× |
| 100,000 | 0.801 | 124.78M | 0.778 | 128.57M | 0.734 | 0.92× | 0.94× |
| 1,000,000 | 9.271 | 107.86M | 8.656 | 115.53M | 6.889 | 0.74× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.132 | 1.12× |
| 1 | 5 | 0.252 | 0.524 | 2.08× |
| 1 | 10 | 0.470 | 0.946 | 2.01× |
| 10 | 1 | 0.045 | 0.089 | 1.96× |
| 10 | 5 | 0.228 | 0.467 | 2.05× |
| 10 | 10 | 0.511 | 0.984 | 1.93× |
| 100 | 1 | 0.054 | 0.101 | 1.87× |
| 100 | 5 | 0.236 | 0.446 | 1.90× |
| 100 | 10 | 0.491 | 1.019 | 2.07× |
| 1,000 | 1 | 0.058 | 0.100 | 1.72× |
| 1,000 | 5 | 0.241 | 0.471 | 1.96× |
| 1,000 | 10 | 0.500 | 1.030 | 2.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
