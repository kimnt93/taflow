# MathAtan benchmark (`ATAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 110.06M | 0.008 | 121.58M | 0.035 | 3.82× | 4.22× |
| 10,000 | 0.062 | 160.08M | 0.059 | 170.71M | 0.086 | 1.37× | 1.47× |
| 100,000 | 0.586 | 170.62M | 0.562 | 177.96M | 0.606 | 1.03× | 1.08× |
| 1,000,000 | 6.492 | 154.03M | 5.871 | 170.34M | 5.988 | 0.92× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.109 | 1.34× |
| 1 | 5 | 0.273 | 0.538 | 1.97× |
| 1 | 10 | 0.523 | 1.065 | 2.04× |
| 10 | 1 | 0.049 | 0.086 | 1.76× |
| 10 | 5 | 0.236 | 0.454 | 1.92× |
| 10 | 10 | 0.497 | 1.014 | 2.04× |
| 100 | 1 | 0.057 | 0.099 | 1.73× |
| 100 | 5 | 0.253 | 0.479 | 1.89× |
| 100 | 10 | 0.517 | 0.994 | 1.92× |
| 1,000 | 1 | 0.058 | 0.095 | 1.64× |
| 1,000 | 5 | 0.253 | 0.483 | 1.91× |
| 1,000 | 10 | 0.546 | 1.038 | 1.90× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.293 | 0.161 | 6.22M | 632.725 | 3937.30× | 172.46× |
| 100,000 | 10 | 0.993 | 0.575 | 17.38M | 572.540 | 995.14× | 47.08× |
| 100,000 | 1,000 | 8.417 | 6.845 | 146.10M | 583.120 | 85.19× | 4.82× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 128.49M | 151.71M | 1.00× | 2.89M | 4.11M | 1.00× | 141.53M |
| 5 | 379.47M | 576.55M | 3.80× | 2.21M | 2.78M | 0.68× | 134.08M |
| 10 | 492.83M | 671.01M | 4.42× | 2.02M | 2.51M | 0.61× | 135.13M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
