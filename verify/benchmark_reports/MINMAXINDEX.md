# RollingMinMaxIndex benchmark (`MINMAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.00M | 0.019 | 54.05M | 0.046 | 2.24× | 2.47× |
| 10,000 | 0.274 | 36.44M | 0.284 | 35.15M | 0.154 | 0.56× | 0.54× |
| 100,000 | 3.060 | 32.68M | 2.648 | 37.77M | 1.239 | 0.41× | 0.47× |
| 1,000,000 | 29.538 | 33.85M | 30.464 | 32.83M | 11.794 | 0.40× | 0.39× |

## Warm-up

Construct + canonical extend over 100,000 bars: **3.247 ms**; native kernel **2.636 ms**; TA-Lib 1.231 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.382 | 0.290 | 3.45M | 1273.705 | 4398.52× | 122.49× |
| 100,000 | 10 | 2.639 | 1.363 | 7.33M | 1232.275 | 903.79× | 26.25× |
| 100,000 | 1,000 | 127.462 | 108.240 | 9.24M | 1261.277 | 11.65× | 0.46× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 30.12M | 34.94M | 1.00× | 1.58M | 1.90M | 1.00× | 68.48M |
| 2 | 59.23M | 69.11M | 1.98× | 1.64M | 2.25M | 1.18× | 68.75M |
| 4 | 71.30M | 78.38M | 2.24× | 1.79M | 2.02M | 1.06× | 71.74M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
