# CandleRiseFallThreeMethods benchmark (`CDLRISEFALL3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 110.20M | 0.007 | 141.35M | 0.034 | 3.76× | 4.83× |
| 10,000 | 0.110 | 90.52M | 0.099 | 100.87M | 0.125 | 1.13× | 1.26× |
| 100,000 | 1.254 | 79.74M | 1.128 | 88.64M | 1.060 | 0.84× | 0.94× |
| 1,000,000 | 11.905 | 84.00M | 11.860 | 84.32M | 10.883 | 0.91× | 0.92× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.087 ms**; native kernel **1.084 ms**; TA-Lib 0.943 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.360 | 0.300 | 3.33M | 940.586 | 3135.16× | 96.82× |
| 100,000 | 10 | 2.734 | 1.534 | 6.52M | 943.786 | 615.05× | 18.48× |
| 100,000 | 1,000 | 33.755 | 31.064 | 32.19M | 960.765 | 30.93× | 1.24× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 82.44M | 77.53M | 1.00× | 2.10M | 2.24M | 1.00× | 82.60M |
| 2 | 147.90M | 159.97M | 2.06× | 2.15M | 2.54M | 1.13× | 82.66M |
| 4 | 279.19M | 301.13M | 3.88× | 2.34M | 2.55M | 1.14× | 86.67M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
