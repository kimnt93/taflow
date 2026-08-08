# CandleRickshawman benchmark (`CDLRICKSHAWMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 132.88M | 0.005 | 187.58M | 0.039 | 5.12× | 7.23× |
| 10,000 | 0.060 | 166.57M | 0.057 | 174.48M | 0.130 | 2.16× | 2.26× |
| 100,000 | 0.700 | 142.86M | 0.686 | 145.75M | 1.081 | 1.54× | 1.58× |
| 1,000,000 | 8.365 | 119.55M | 7.523 | 132.93M | 10.430 | 1.25× | 1.39× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.726 ms**; native kernel **0.690 ms**; TA-Lib 1.081 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.408 | 0.318 | 3.14M | 1165.148 | 3659.24× | 96.13× |
| 100,000 | 10 | 3.515 | 1.476 | 6.77M | 1105.952 | 749.08× | 26.38× |
| 100,000 | 1,000 | 59.136 | 32.815 | 30.47M | 1101.308 | 33.56× | 1.16× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 106.57M | 121.72M | 1.00× | 2.21M | 2.51M | 1.00× | 83.60M |
| 2 | 237.52M | 247.45M | 2.03× | 2.20M | 2.37M | 0.94× | 84.68M |
| 4 | 378.33M | 385.76M | 3.17× | 2.40M | 2.33M | 0.93× | 84.99M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
