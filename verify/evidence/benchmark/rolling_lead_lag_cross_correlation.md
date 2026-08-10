# RollingLeadLagCrossCorrelation benchmark (`LeadLagCrossCorrelation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.967 | 1.03M | 0.921 | 1.09M | 1.492 | 1.54× | 1.62× |
| 10,000 | 9.442 | 1.06M | 9.634 | 1.04M | 13.344 | 1.41× | 1.39× |
| 100,000 | 94.310 | 1.06M | 103.452 | 966.63K | 130.718 | 1.39× | 1.26× |
| 1,000,000 | 993.407 | 1.01M | 1010.716 | 989.40K | 1375.943 | 1.39× | 1.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.139 | 0.309 | 2.22× |
| 1 | 5 | 0.320 | 1.460 | 4.56× |
| 1 | 10 | 0.551 | 2.810 | 5.10× |
| 10 | 1 | 0.059 | 0.264 | 4.51× |
| 10 | 5 | 0.228 | 1.557 | 6.83× |
| 10 | 10 | 0.498 | 2.845 | 5.71× |
| 100 | 1 | 0.121 | 0.385 | 3.19× |
| 100 | 5 | 0.250 | 2.003 | 8.00× |
| 100 | 10 | 0.608 | 3.935 | 6.48× |
| 1,000 | 1 | 1.002 | 1.777 | 1.77× |
| 1,000 | 5 | 1.420 | 8.331 | 5.87× |
| 1,000 | 10 | 2.022 | 23.949 | 11.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
