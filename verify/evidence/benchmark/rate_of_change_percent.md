# RateOfChangePercent benchmark (`ROCP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 460.85M | 0.001 | 705.47M | 0.030 | 13.90× | 21.27× |
| 10,000 | 0.008 | 1.20G | 0.006 | 1.79G | 0.038 | 4.54× | 6.76× |
| 100,000 | 0.070 | 1.43G | 0.047 | 2.14G | 0.118 | 1.69× | 2.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.115 | 1.38× |
| 1 | 5 | 0.249 | 0.573 | 2.30× |
| 1 | 10 | 0.411 | 0.917 | 2.23× |
| 10 | 1 | 0.043 | 0.091 | 2.15× |
| 10 | 5 | 0.182 | 0.452 | 2.48× |
| 10 | 10 | 0.435 | 1.181 | 2.72× |
| 100 | 1 | 0.050 | 0.098 | 1.95× |
| 100 | 5 | 0.201 | 0.445 | 2.21× |
| 100 | 10 | 0.386 | 0.907 | 2.35× |
| 1,000 | 1 | 0.045 | 0.095 | 2.09× |
| 1,000 | 5 | 0.178 | 0.441 | 2.47× |
| 1,000 | 10 | 0.377 | 0.905 | 2.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
