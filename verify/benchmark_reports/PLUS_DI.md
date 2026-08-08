# PlusDirectionalIndicator benchmark (`PLUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.03M | 0.013 | 79.35M | 0.039 | 2.76× | 3.08× |
| 10,000 | 0.129 | 77.66M | 0.114 | 87.78M | 0.097 | 0.75× | 0.85× |
| 100,000 | 1.133 | 88.29M | 1.111 | 90.02M | 0.711 | 0.63× | 0.64× |
| 1,000,000 | 11.684 | 85.59M | 11.470 | 87.18M | 6.915 | 0.59× | 0.60× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.137 ms**; native kernel **1.107 ms**; TA-Lib 0.703 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.281 | 0.219 | 4.56M | 676.187 | 3086.61× | 142.94× |
| 100,000 | 10 | 2.063 | 1.128 | 8.87M | 673.105 | 596.90× | 26.12× |
| 100,000 | 1,000 | 17.414 | 12.909 | 77.46M | 676.883 | 52.43× | 2.89× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 73.41M | 76.58M | 1.00× | 2.55M | 2.52M | 1.00× | 116.18M |
| 2 | 145.98M | 142.65M | 1.86× | 2.35M | 3.03M | 1.20× | 118.81M |
| 4 | 229.61M | 251.43M | 3.28× | 2.38M | 2.41M | 0.96× | 119.40M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
