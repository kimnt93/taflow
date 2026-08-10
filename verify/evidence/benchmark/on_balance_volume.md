# OnBalanceVolume benchmark (`OBV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 114.96M | 0.008 | 127.82M | 0.033 | 3.85× | 4.28× |
| 10,000 | 0.066 | 152.31M | 0.062 | 161.82M | 0.066 | 1.00× | 1.07× |
| 100,000 | 0.632 | 158.25M | 0.613 | 163.24M | 0.402 | 0.64× | 0.66× |
| 1,000,000 | 6.633 | 150.77M | 6.014 | 166.27M | 3.973 | 0.60× | 0.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.117 | 1.09× |
| 1 | 5 | 0.260 | 0.457 | 1.76× |
| 1 | 10 | 0.486 | 1.042 | 2.15× |
| 10 | 1 | 0.056 | 0.091 | 1.63× |
| 10 | 5 | 0.293 | 0.520 | 1.78× |
| 10 | 10 | 0.497 | 0.973 | 1.96× |
| 100 | 1 | 0.054 | 0.093 | 1.71× |
| 100 | 5 | 0.232 | 0.538 | 2.32× |
| 100 | 10 | 0.516 | 0.974 | 1.89× |
| 1,000 | 1 | 0.055 | 0.089 | 1.64× |
| 1,000 | 5 | 0.256 | 0.468 | 1.83× |
| 1,000 | 10 | 0.586 | 1.139 | 1.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
