# CandleStalledPattern benchmark (`CDLSTALLEDPATTERN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.34M | 0.007 | 144.84M | 0.040 | 4.35× | 5.86× |
| 10,000 | 0.089 | 112.50M | 0.084 | 119.36M | 0.164 | 1.84× | 1.95× |
| 100,000 | 1.007 | 99.27M | 0.929 | 107.63M | 1.414 | 1.40× | 1.52× |
| 1,000,000 | 9.997 | 100.03M | 9.820 | 101.83M | 13.961 | 1.40× | 1.42× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.943 ms**; native kernel **0.891 ms**; TA-Lib 1.436 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.381 | 0.292 | 3.43M | 1419.040 | 4867.79× | 95.67× |
| 100,000 | 10 | 2.796 | 1.499 | 6.67M | 1414.991 | 943.69× | 19.20× |
| 100,000 | 1,000 | 44.154 | 30.049 | 33.28M | 1463.205 | 48.69× | 1.40× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 86.23M | 95.40M | 1.00× | 2.19M | 2.07M | 1.00× | 63.00M |
| 2 | 173.62M | 182.46M | 1.91× | 2.12M | 2.44M | 1.18× | 65.32M |
| 4 | 306.48M | 363.92M | 3.81× | 2.35M | 2.59M | 1.25× | 65.73M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
