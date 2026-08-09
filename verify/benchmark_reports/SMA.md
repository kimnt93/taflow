# SimpleMovingAverage benchmark (`SMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 184.71M | 0.004 | 223.55M | 0.034 | 6.24× | 7.55× |
| 10,000 | 0.026 | 387.13M | 0.022 | 445.03M | 0.060 | 2.31× | 2.65× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.006 ms**; TA-Lib 0.036 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.464 | 0.184 | 5.43M | 36.911 | 200.45× | 172.35× |
| 1,500 | 10 | 1.128 | 0.615 | 16.26M | 35.112 | 57.08× | 53.95× |
| 1,500 | 100 | 3.071 | 1.912 | 52.29M | 34.693 | 18.14× | 17.56× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.21M | 12.39M | 1.00× | 1.26M | 1.45M | 1.00× | 6.76M |
| 2 | 20.34M | 18.45M | 1.49× | 1.34M | 1.62M | 1.12× | 8.92M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
