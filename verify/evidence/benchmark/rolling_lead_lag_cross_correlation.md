# RollingLeadLagCrossCorrelation benchmark (`LeadLagCrossCorrelation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 1.112 | 899.67K | 1.028 | 972.87K | 1.516 | 1.36× | 1.47× |
| 10,000 | 9.853 | 1.01M | 10.920 | 915.74K | 14.081 | 1.43× | 1.29× |
| 100,000 | 95.203 | 1.05M | 95.768 | 1.04M | 132.341 | 1.39× | 1.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.320 | 4.15× |
| 1 | 5 | 0.247 | 7.881 | 31.91× |
| 1 | 10 | 0.562 | 2.663 | 4.74× |
| 10 | 1 | 0.047 | 0.250 | 5.31× |
| 10 | 5 | 0.195 | 1.448 | 7.44× |
| 10 | 10 | 0.411 | 2.615 | 6.36× |
| 100 | 1 | 0.105 | 0.366 | 3.50× |
| 100 | 5 | 0.215 | 1.899 | 8.83× |
| 100 | 10 | 0.470 | 3.771 | 8.03× |
| 1,000 | 1 | 0.984 | 1.735 | 1.76× |
| 1,000 | 5 | 1.103 | 8.451 | 7.66× |
| 1,000 | 10 | 1.859 | 16.796 | 9.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
