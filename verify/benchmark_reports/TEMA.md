# TripleExponentialMovingAverage benchmark (`TEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.43M | 0.006 | 179.60M | 0.041 | 5.92× | 7.37× |
| 10,000 | 0.048 | 209.86M | 0.045 | 224.23M | 0.122 | 2.56× | 2.74× |
| 100,000 | 0.498 | 200.95M | 0.420 | 238.08M | 0.968 | 1.94× | 2.30× |
| 1,000,000 | 4.712 | 212.21M | 4.292 | 233.02M | 10.030 | 2.13× | 2.34× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.440 ms**; native kernel **0.420 ms**; TA-Lib 0.926 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.257 | 0.197 | 5.07M | 910.894 | 4617.75× | 159.46× |
| 100,000 | 10 | 0.886 | 0.526 | 19.00M | 923.424 | 1754.68× | 59.86× |
| 100,000 | 1,000 | 10.167 | 5.431 | 184.11M | 954.974 | 175.82× | 7.41× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 159.63M | 185.09M | 1.00× | 3.26M | 3.16M | 1.00× | 94.56M |
| 2 | 314.30M | 317.05M | 1.71× | 3.27M | 3.67M | 1.16× | 93.81M |
| 4 | 460.51M | 616.12M | 3.33× | 3.00M | 3.34M | 1.06× | 93.45M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
