# MovingAverageConvergenceDivergenceFixed benchmark (`MACDFIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 154.41M | 0.005 | 192.58M | 0.046 | 7.06× | 8.81× |
| 10,000 | 0.049 | 203.21M | 0.044 | 225.94M | 0.129 | 2.63× | 2.93× |
| 100,000 | 0.468 | 213.62M | 0.401 | 249.67M | 0.982 | 2.10× | 2.45× |
| 1,000,000 | 16.519 | 60.53M | 4.229 | 236.44M | 18.800 | 1.14× | 4.44× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.465 ms**; native kernel **0.388 ms**; TA-Lib 0.987 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.326 | 0.247 | 4.04M | 945.489 | 3823.89× | 149.71× |
| 100,000 | 10 | 1.418 | 1.402 | 7.13M | 989.716 | 705.88× | 27.38× |
| 100,000 | 1,000 | 89.747 | 70.879 | 14.11M | 970.312 | 13.69× | 0.67× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 157.96M | 180.55M | 1.00× | 1.87M | 2.01M | 1.00× | 77.04M |
| 2 | 233.18M | 343.32M | 1.90× | 1.56M | 1.65M | 0.82× | 78.84M |
| 4 | 242.41M | 420.55M | 2.33× | 1.42M | 1.62M | 0.81× | 79.75M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
