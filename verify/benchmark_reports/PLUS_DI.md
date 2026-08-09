# PlusDirectionalIndicator benchmark (`PLUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 92.08M | 0.009 | 107.67M | 0.037 | 3.44× | 4.03× |
| 10,000 | 0.062 | 160.68M | 0.061 | 163.04M | 0.095 | 1.52× | 1.54× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.013 ms**; TA-Lib 0.050 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.312 | 0.243 | 4.12M | 40.887 | 168.28× | 134.85× |
| 1,500 | 10 | 2.039 | 1.035 | 9.66M | 41.695 | 40.29× | 30.87× |
| 1,500 | 100 | 4.412 | 2.668 | 37.48M | 41.644 | 15.61× | 11.89× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.55M | 8.33M | 1.00× | 1.25M | 1.33M | 1.00× | 9.02M |
| 2 | 17.08M | 16.29M | 1.96× | 1.48M | 1.58M | 1.19× | 9.35M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
