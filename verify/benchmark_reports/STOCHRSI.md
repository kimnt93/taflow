# StochasticRelativeStrengthIndex benchmark (`STOCHRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.63M | 0.020 | 49.75M | 0.053 | 2.51× | 2.63× |
| 10,000 | 0.224 | 44.59M | 0.223 | 44.82M | 0.195 | 0.87× | 0.88× |
| 100,000 | 2.208 | 45.30M | 2.148 | 46.56M | 1.523 | 0.69× | 0.71× |
| 1,000,000 | 30.965 | 32.29M | 30.704 | 32.57M | 14.929 | 0.48× | 0.49× |

## Warm-up

Construct + canonical extend over 100,000 bars: **2.185 ms**; native kernel **2.157 ms**; TA-Lib 1.494 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.311 | 0.268 | 3.74M | 1501.829 | 5614.12× | 158.91× |
| 100,000 | 10 | 1.846 | 1.518 | 6.59M | 1503.765 | 990.82× | 28.13× |
| 100,000 | 1,000 | 95.046 | 95.007 | 10.53M | 1492.520 | 15.71× | 0.56× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 39.50M | 42.07M | 1.00× | 2.30M | 2.08M | 1.00× | 56.96M |
| 2 | 74.28M | 80.34M | 1.91× | 1.93M | 2.33M | 1.12× | 54.99M |
| 4 | 116.51M | 147.23M | 3.50× | 1.91M | 2.02M | 0.97× | 55.53M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
