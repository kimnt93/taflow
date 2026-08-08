# RollingVariance benchmark (`VAR` oracle)

Correctness: **MISMATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 222.62M | 0.003 | 289.67M | 0.034 | 7.56× | 9.84× |
| 10,000 | 0.028 | 353.19M | 0.027 | 375.01M | 0.053 | 1.89× | 2.00× |
| 100,000 | 0.273 | 365.79M | 0.248 | 403.75M | 0.235 | 0.86× | 0.95× |
| 1,000,000 | 2.862 | 349.36M | 2.504 | 399.33M | 2.149 | 0.75× | 0.86× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.268 ms**; native kernel **0.244 ms**; TA-Lib 0.236 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.225 | 0.146 | 6.83M | 242.804 | 1658.34× | 217.22× |
| 100,000 | 10 | 0.880 | 0.524 | 19.09M | 244.013 | 465.74× | 57.77× |
| 100,000 | 1,000 | 5.136 | 3.775 | 264.93M | 238.045 | 63.07× | 9.19× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 255.52M | 310.42M | 1.00× | 3.54M | 3.87M | 1.00× | 278.33M |
| 2 | 475.45M | 610.50M | 1.97× | 3.07M | 3.87M | 1.00× | 301.63M |
| 4 | 572.81M | 816.08M | 2.63× | 2.80M | 3.32M | 0.86× | 292.77M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
