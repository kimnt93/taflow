# CandleRiseFallThreeMethods benchmark (`CDLRISEFALL3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 113.80M | 0.007 | 144.09M | 0.034 | 3.89× | 4.92× |
| 10,000 | 0.093 | 107.07M | 0.089 | 112.56M | 0.114 | 1.22× | 1.28× |
| 100,000 | 1.003 | 99.71M | 0.986 | 101.42M | 0.913 | 0.91× | 0.93× |
| 1,000,000 | 10.349 | 96.63M | 10.194 | 98.10M | 9.962 | 0.96× | 0.98× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.996 ms**; native kernel **0.985 ms**; TA-Lib 0.930 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.341 | 0.280 | 3.57M | 906.049 | 3231.53× | 99.48× |
| 100,000 | 10 | 2.673 | 1.376 | 7.27M | 904.133 | 657.24× | 20.48× |
| 100,000 | 1,000 | 41.240 | 31.864 | 31.38M | 915.932 | 28.74× | 1.13× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 87.90M | 83.01M | 1.00× | 2.28M | 2.52M | 1.00× | 86.30M |
| 2 | 172.13M | 172.71M | 2.08× | 2.38M | 2.53M | 1.00× | 90.96M |
| 4 | 295.51M | 326.97M | 3.94× | 2.20M | 2.42M | 0.96× | 88.58M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
