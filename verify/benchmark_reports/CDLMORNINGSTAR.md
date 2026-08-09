# CandleMorningStar benchmark (`CDLMORNINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.20M | 0.009 | 110.67M | 0.038 | 3.69× | 4.21× |
| 10,000 | 0.082 | 121.41M | 0.078 | 127.83M | 0.110 | 1.34× | 1.41× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.011 ms**; TA-Lib 0.040 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.344 | 0.270 | 3.71M | 40.695 | 150.97× | 120.25× |
| 1,500 | 10 | 2.535 | 1.255 | 7.97M | 41.821 | 33.32× | 26.31× |
| 1,500 | 100 | 5.552 | 3.497 | 28.60M | 41.385 | 11.83× | 9.57× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.89M | 13.35M | 1.00× | 751.06K | 1.02M | 1.00× | 8.60M |
| 2 | 12.09M | 19.05M | 1.43× | 1.32M | 1.12M | 1.09× | 9.27M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
