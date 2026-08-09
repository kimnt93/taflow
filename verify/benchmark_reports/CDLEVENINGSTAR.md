# CandleEveningStar benchmark (`CDLEVENINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 44.40M | 0.017 | 59.20M | 0.072 | 3.21× | 4.29× |
| 10,000 | 0.156 | 64.23M | 0.152 | 65.73M | 0.208 | 1.34× | 1.37× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.026 ms**; native kernel **0.012 ms**; TA-Lib 0.048 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.439 | 0.339 | 2.95M | 53.677 | 158.41× | 124.54× |
| 1,500 | 10 | 5.593 | 2.672 | 3.74M | 47.468 | 17.76× | 15.07× |
| 1,500 | 100 | 6.651 | 3.897 | 25.66M | 49.965 | 12.82× | 10.04× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.97M | 12.26M | 1.00× | 1.07M | 1.23M | 1.00× | 7.92M |
| 2 | 12.96M | 16.90M | 1.38× | 1.05M | 1.16M | 0.94× | 6.92M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
