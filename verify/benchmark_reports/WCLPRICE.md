# WeightedClose benchmark (`WCLPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 352.36M | 0.001 | 720.14M | 0.029 | 10.04× | 20.53× |
| 10,000 | 0.010 | 1.00G | 0.006 | 1.62G | 0.034 | 3.36× | 5.45× |
| 100,000 | 0.078 | 1.27G | 0.054 | 1.87G | 0.083 | 1.06× | 1.55× |
| 1,000,000 | 1.491 | 670.91M | 1.125 | 888.61M | 1.157 | 0.78× | 1.03× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.079 ms**; native kernel **0.053 ms**; TA-Lib 0.082 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.294 | 0.219 | 4.56M | 83.614 | 381.18× | 121.39× |
| 100,000 | 10 | 1.846 | 0.865 | 11.56M | 81.717 | 94.50× | 31.69× |
| 100,000 | 1,000 | 4.510 | 2.542 | 393.39M | 84.850 | 33.38× | 11.06× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 421.69M | 699.40M | 1.00× | 2.84M | 3.09M | 1.00× | 560.10M |
| 2 | 802.81M | 1.24G | 1.77× | 2.80M | 2.87M | 0.93× | 653.38M |
| 4 | 885.33M | 1.77G | 2.54× | 2.78M | 2.71M | 0.88× | 505.17M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
