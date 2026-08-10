# RateOfChange benchmark (`ROC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 221.10M | 0.004 | 263.20M | 0.030 | 6.69× | 7.96× |
| 10,000 | 0.020 | 488.01M | 0.018 | 541.31M | 0.039 | 1.91× | 2.12× |
| 100,000 | 0.181 | 552.17M | 0.164 | 610.28M | 0.127 | 0.70× | 0.77× |
| 1,000,000 | 2.106 | 474.75M | 1.655 | 604.11M | 1.097 | 0.52× | 0.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.124 | 1.52× |
| 1 | 5 | 0.249 | 0.456 | 1.83× |
| 1 | 10 | 0.484 | 0.931 | 1.92× |
| 10 | 1 | 0.048 | 0.093 | 1.95× |
| 10 | 5 | 0.223 | 0.439 | 1.97× |
| 10 | 10 | 0.532 | 0.943 | 1.77× |
| 100 | 1 | 0.050 | 0.091 | 1.82× |
| 100 | 5 | 0.225 | 0.438 | 1.94× |
| 100 | 10 | 0.492 | 0.913 | 1.85× |
| 1,000 | 1 | 0.054 | 0.088 | 1.63× |
| 1,000 | 5 | 0.224 | 0.433 | 1.93× |
| 1,000 | 10 | 0.514 | 0.929 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
