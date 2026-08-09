# TriangularMovingAverage benchmark (`TRIMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 133.02M | 0.007 | 153.77M | 0.033 | 4.44× | 5.13× |
| 10,000 | 0.045 | 224.51M | 0.042 | 237.74M | 0.061 | 1.38× | 1.46× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.009 ms**; TA-Lib 0.035 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.302 | 0.176 | 5.69M | 35.107 | 199.59× | 173.24× |
| 1,500 | 10 | 1.138 | 0.637 | 15.71M | 35.508 | 55.77× | 48.73× |
| 1,500 | 100 | 3.235 | 2.039 | 49.05M | 35.410 | 17.37× | 15.10× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.68M | 13.48M | 1.00× | 969.15K | 1.15M | 1.00× | 9.20M |
| 2 | 13.64M | 20.15M | 1.49× | 1.45M | 1.75M | 1.52× | 9.20M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
