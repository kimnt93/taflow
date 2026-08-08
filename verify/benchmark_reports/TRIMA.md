# TriangularMovingAverage benchmark (`TRIMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.50M | 0.005 | 192.19M | 0.034 | 5.60× | 6.55× |
| 10,000 | 0.045 | 220.91M | 0.042 | 237.71M | 0.061 | 1.35× | 1.46× |
| 100,000 | 0.419 | 238.82M | 0.401 | 249.26M | 0.315 | 0.75× | 0.79× |
| 1,000,000 | 4.557 | 219.43M | 4.210 | 237.56M | 2.944 | 0.65× | 0.70× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.420 ms**; native kernel **0.405 ms**; TA-Lib 0.327 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.230 | 0.153 | 6.52M | 319.450 | 2083.35× | 193.26× |
| 100,000 | 10 | 1.010 | 0.573 | 17.44M | 333.981 | 582.36× | 54.52× |
| 100,000 | 1,000 | 9.281 | 8.151 | 122.68M | 329.038 | 40.37× | 4.15× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 178.29M | 187.10M | 1.00× | 3.03M | 3.83M | 1.00× | 238.17M |
| 2 | 315.09M | 375.45M | 2.01× | 3.34M | 3.31M | 0.86× | 217.60M |
| 4 | 510.69M | 695.10M | 3.72× | 3.21M | 3.42M | 0.89× | 234.70M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
