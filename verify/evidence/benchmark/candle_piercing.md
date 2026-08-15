# CandlePiercing benchmark (`CDLPIERCING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 82.34M | 0.008 | 118.29M | 0.032 | 2.65× | 3.80× |
| 10,000 | 0.121 | 82.62M | 0.119 | 83.74M | 0.114 | 0.94× | 0.96× |
| 100,000 | 1.196 | 83.63M | 1.283 | 77.91M | 1.015 | 0.85× | 0.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.105 | 0.89× |
| 1 | 5 | 0.301 | 0.454 | 1.51× |
| 1 | 10 | 0.395 | 0.915 | 2.32× |
| 10 | 1 | 0.043 | 0.097 | 2.27× |
| 10 | 5 | 0.197 | 0.470 | 2.38× |
| 10 | 10 | 0.468 | 0.929 | 1.98× |
| 100 | 1 | 0.045 | 0.089 | 1.98× |
| 100 | 5 | 0.187 | 0.461 | 2.47× |
| 100 | 10 | 0.439 | 0.938 | 2.14× |
| 1,000 | 1 | 0.057 | 0.097 | 1.70× |
| 1,000 | 5 | 0.205 | 0.468 | 2.28× |
| 1,000 | 10 | 0.416 | 1.108 | 2.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
