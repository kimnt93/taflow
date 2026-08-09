# BalanceOfPower benchmark (`BOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 124.58M | 0.006 | 171.24M | 0.028 | 3.50× | 4.81× |
| 10,000 | 0.019 | 515.32M | 0.016 | 613.81M | 0.037 | 1.93× | 2.30× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.006 ms**; TA-Lib 0.028 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.333 | 0.255 | 3.91M | 28.510 | 111.59× | 102.51× |
| 1,500 | 10 | 2.917 | 1.215 | 8.23M | 30.285 | 24.93× | 21.57× |
| 1,500 | 100 | 4.514 | 2.551 | 39.20M | 28.534 | 11.19× | 10.36× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.81M | 15.10M | 1.00× | 873.34K | 983.93K | 1.00× | 8.77M |
| 2 | 18.02M | 17.13M | 1.13× | 1.33M | 1.42M | 1.44× | 9.67M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
