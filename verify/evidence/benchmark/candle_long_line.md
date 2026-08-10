# CandleLongLine benchmark (`CDLLONGLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.89M | 0.016 | 61.88M | 0.033 | 1.72× | 2.05× |
| 10,000 | 0.158 | 63.21M | 0.151 | 66.39M | 0.177 | 1.12× | 1.17× |
| 100,000 | 1.527 | 65.49M | 1.551 | 64.48M | 1.430 | 0.94× | 0.92× |
| 1,000,000 | 15.642 | 63.93M | 15.708 | 63.66M | 14.514 | 0.93× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.122 | 1.38× |
| 1 | 5 | 0.297 | 0.442 | 1.49× |
| 1 | 10 | 0.543 | 0.923 | 1.70× |
| 10 | 1 | 0.058 | 0.090 | 1.55× |
| 10 | 5 | 0.238 | 0.427 | 1.80× |
| 10 | 10 | 0.538 | 0.889 | 1.65× |
| 100 | 1 | 0.055 | 0.089 | 1.61× |
| 100 | 5 | 0.240 | 0.411 | 1.71× |
| 100 | 10 | 0.502 | 0.898 | 1.79× |
| 1,000 | 1 | 0.066 | 0.102 | 1.56× |
| 1,000 | 5 | 0.247 | 0.516 | 2.09× |
| 1,000 | 10 | 0.575 | 1.043 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
