# CandleLadderBottom benchmark (`CDLLADDERBOTTOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 162.39M | 0.004 | 243.61M | 0.031 | 5.09× | 7.64× |
| 10,000 | 0.062 | 161.57M | 0.059 | 169.98M | 0.087 | 1.40× | 1.47× |
| 100,000 | 0.658 | 151.92M | 0.652 | 153.34M | 0.592 | 0.90× | 0.91× |
| 1,000,000 | 7.011 | 142.64M | 6.935 | 144.19M | 6.008 | 0.86× | 0.87× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.662 ms**; native kernel **0.643 ms**; TA-Lib 0.597 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.348 | 0.319 | 3.13M | 583.293 | 1828.37× | 87.25× |
| 100,000 | 10 | 2.886 | 1.432 | 6.98M | 582.853 | 406.97× | 19.67× |
| 100,000 | 1,000 | 29.444 | 32.690 | 30.59M | 579.301 | 17.72× | 0.97× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 128.36M | 135.62M | 1.00× | 2.43M | 2.37M | 1.00× | 142.38M |
| 2 | 236.97M | 257.34M | 1.90× | 2.27M | 2.58M | 1.09× | 141.53M |
| 4 | 433.06M | 471.72M | 3.48× | 2.29M | 2.68M | 1.13× | 139.90M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
