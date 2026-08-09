# ExponentialMovingAverage benchmark (`EMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 233.18M | 0.003 | 308.25M | 0.034 | 7.88× | 10.41× |
| 10,000 | 0.027 | 370.58M | 0.027 | 367.78M | 0.057 | 2.12× | 2.11× |
| 100,000 | 0.257 | 388.83M | 0.232 | 431.30M | 0.298 | 1.16× | 1.28× |
| 1,000,000 | 3.466 | 288.50M | 2.963 | 337.48M | 2.742 | 0.79× | 0.93× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.258 ms**; native kernel **0.233 ms**; TA-Lib 0.313 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.223 | 0.155 | 6.47M | 299.039 | 1935.25× | 193.20× |
| 100,000 | 10 | 0.884 | 0.507 | 19.72M | 295.515 | 582.75× | 60.27× |
| 100,000 | 1,000 | 6.249 | 4.864 | 205.59M | 305.313 | 62.77× | 6.73× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 226.14M | 232.07M | 1.00× | 2.84M | 4.30M | 1.00× | 234.36M |
| 2 | 444.39M | 485.54M | 2.09× | 3.46M | 3.52M | 0.82× | 225.82M |
| 4 | 417.71M | 701.27M | 3.02× | 3.15M | 3.28M | 0.76× | 233.64M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
