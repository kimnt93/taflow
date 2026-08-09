# MathAsin benchmark (`ASIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.54M | 0.009 | 114.78M | 0.032 | 3.15× | 3.67× |
| 10,000 | 0.073 | 136.81M | 0.072 | 139.23M | 0.088 | 1.21× | 1.23× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.012 ms**; TA-Lib 0.036 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.307 | 0.181 | 5.53M | 34.843 | 192.69× | 140.93× |
| 1,500 | 10 | 1.210 | 0.643 | 15.56M | 34.628 | 53.88× | 39.82× |
| 1,500 | 100 | 3.732 | 2.455 | 40.73M | 35.369 | 14.40× | 10.25× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.97M | 7.99M | 1.00× | 1.27M | 1.41M | 1.00× | 8.07M |
| 2 | 14.22M | 21.51M | 2.69× | 979.68K | 1.66M | 1.17× | 9.49M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
