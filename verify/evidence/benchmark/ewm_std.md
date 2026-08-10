# ExponentiallyWeightedStandardDeviation benchmark (`ewm standard deviation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.82M | 0.007 | 151.48M | 1.950 | 260.96× | 295.39× |
| 10,000 | 0.051 | 197.13M | 0.048 | 209.01M | 12.549 | 247.38× | 262.29× |
| 100,000 | 0.519 | 192.69M | 0.409 | 244.52M | 127.634 | 245.94× | 312.09× |
| 1,000,000 | 4.760 | 210.10M | 4.215 | 237.25M | 1249.264 | 262.47× | 296.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.146 | 1.51× |
| 1 | 5 | 0.320 | 0.434 | 1.36× |
| 1 | 10 | 0.428 | 0.833 | 1.95× |
| 10 | 1 | 0.050 | 0.097 | 1.94× |
| 10 | 5 | 0.214 | 0.484 | 2.26× |
| 10 | 10 | 0.454 | 0.988 | 2.18× |
| 100 | 1 | 0.045 | 0.219 | 4.84× |
| 100 | 5 | 0.227 | 1.047 | 4.62× |
| 100 | 10 | 0.481 | 2.103 | 4.37× |
| 1,000 | 1 | 0.054 | 1.352 | 24.92× |
| 1,000 | 5 | 0.232 | 6.744 | 29.04× |
| 1,000 | 10 | 0.533 | 13.999 | 26.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
