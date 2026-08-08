# CandleShortLine benchmark (`CDLSHORTLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 141.16M | 0.005 | 193.18M | 0.036 | 5.07× | 6.93× |
| 10,000 | 0.107 | 93.37M | 0.104 | 96.60M | 0.205 | 1.91× | 1.98× |
| 100,000 | 1.182 | 84.63M | 1.199 | 83.42M | 1.750 | 1.48× | 1.46× |
| 1,000,000 | 12.382 | 80.76M | 12.365 | 80.88M | 17.384 | 1.40× | 1.41× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.164 ms**; native kernel **1.161 ms**; TA-Lib 1.738 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.342 | 0.278 | 3.60M | 1749.077 | 6289.44× | 98.69× |
| 100,000 | 10 | 3.112 | 1.589 | 6.29M | 1746.915 | 1099.57× | 17.63× |
| 100,000 | 1,000 | 33.280 | 27.485 | 36.38M | 1776.219 | 64.63× | 1.22× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 73.85M | 73.98M | 1.00× | 2.00M | 1.92M | 1.00× | 52.86M |
| 2 | 147.36M | 121.34M | 1.64× | 2.21M | 2.07M | 1.08× | 50.40M |
| 4 | 208.05M | 258.30M | 3.49× | 2.19M | 2.34M | 1.21× | 51.40M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
