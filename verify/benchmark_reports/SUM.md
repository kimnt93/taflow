# RollingSum benchmark (`SUM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 158.03M | 0.005 | 185.75M | 0.031 | 4.97× | 5.84× |
| 10,000 | 0.035 | 281.87M | 0.033 | 306.96M | 0.055 | 1.56× | 1.70× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.007 ms**; TA-Lib 0.033 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.338 | 0.161 | 6.20M | 32.708 | 202.94× | 188.61× |
| 1,500 | 10 | 1.090 | 0.590 | 16.94M | 33.445 | 56.66× | 50.00× |
| 1,500 | 100 | 7.773 | 1.948 | 51.34M | 33.551 | 17.23× | 15.56× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.66M | 12.65M | 1.00× | 805.82K | 649.39K | 1.00× | 8.23M |
| 2 | 19.21M | 16.78M | 1.33× | 1.20M | 1.72M | 2.64× | 8.60M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
