# OnBalanceVolume benchmark (`OBV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 318.54M | 0.002 | 539.20M | 0.030 | 9.62× | 16.28× |
| 10,000 | 0.039 | 254.23M | 0.035 | 288.91M | 0.062 | 1.57× | 1.78× |
| 100,000 | 0.452 | 221.36M | 0.414 | 241.65M | 0.366 | 0.81× | 0.88× |
| 1,000,000 | 4.761 | 210.04M | 4.327 | 231.09M | 3.627 | 0.76× | 0.84× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.444 ms**; native kernel **0.416 ms**; TA-Lib 0.367 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.285 | 0.173 | 5.77M | 361.758 | 2087.21× | 166.93× |
| 100,000 | 10 | 1.589 | 0.733 | 13.64M | 376.746 | 513.81× | 37.90× |
| 100,000 | 1,000 | 7.613 | 5.875 | 170.20M | 374.470 | 63.73× | 5.25× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 138.16M | 176.79M | 1.00× | 3.15M | 3.78M | 1.00× | 175.92M |
| 2 | 291.75M | 375.46M | 2.12× | 2.80M | 3.45M | 0.91× | 202.84M |
| 4 | 483.45M | 633.40M | 3.58× | 2.56M | 2.90M | 0.77× | 199.38M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
