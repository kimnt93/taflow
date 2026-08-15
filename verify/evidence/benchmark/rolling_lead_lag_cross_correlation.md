# RollingLeadLagCrossCorrelation benchmark (`LeadLagCrossCorrelation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.869 | 1.15M | 0.906 | 1.10M | 1.455 | 1.67× | 1.61× |
| 10,000 | 9.401 | 1.06M | 9.635 | 1.04M | 13.049 | 1.39× | 1.35× |
| 100,000 | 93.545 | 1.07M | 93.420 | 1.07M | 130.931 | 1.40× | 1.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.318 | 4.11× |
| 1 | 5 | 0.296 | 1.338 | 4.52× |
| 1 | 10 | 0.438 | 2.606 | 5.94× |
| 10 | 1 | 0.043 | 0.253 | 5.84× |
| 10 | 5 | 0.182 | 1.516 | 8.31× |
| 10 | 10 | 0.405 | 2.708 | 6.69× |
| 100 | 1 | 0.115 | 0.349 | 3.03× |
| 100 | 5 | 0.248 | 1.855 | 7.49× |
| 100 | 10 | 0.454 | 3.649 | 8.04× |
| 1,000 | 1 | 0.965 | 1.735 | 1.80× |
| 1,000 | 5 | 1.300 | 8.179 | 6.29× |
| 1,000 | 10 | 2.160 | 16.334 | 7.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
