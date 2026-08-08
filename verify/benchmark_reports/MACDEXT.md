# MovingAverageConvergenceDivergenceExtended benchmark (`MACDEXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.97M | 0.011 | 88.97M | 0.055 | 4.52× | 4.91× |
| 10,000 | 0.112 | 89.17M | 0.097 | 103.62M | 0.113 | 1.01× | 1.17× |
| 100,000 | 0.985 | 101.53M | 0.928 | 107.70M | 0.716 | 0.73× | 0.77× |
| 1,000,000 | 20.813 | 48.05M | 13.137 | 76.12M | 15.243 | 0.73× | 1.16× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.006 ms**; native kernel **0.944 ms**; TA-Lib 0.690 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.348 | 0.301 | 3.32M | 708.928 | 2355.29× | 157.65× |
| 100,000 | 10 | 1.704 | 1.381 | 7.24M | 698.745 | 505.98× | 34.32× |
| 100,000 | 1,000 | 105.707 | 80.346 | 12.45M | 695.991 | 8.66× | 0.69× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 70.54M | 88.79M | 1.00× | 1.65M | 1.55M | 1.00× | 87.41M |
| 2 | 126.81M | 178.37M | 2.01× | 1.79M | 1.57M | 1.01× | 100.30M |
| 4 | 172.97M | 317.27M | 3.57× | 1.42M | 1.32M | 0.85× | 101.83M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
