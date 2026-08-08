# MedianPrice benchmark (`MEDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 398.72M | 0.001 | 865.82M | 0.029 | 11.55× | 25.09× |
| 10,000 | 0.008 | 1.23G | 0.004 | 2.46G | 0.034 | 4.12× | 8.25× |
| 100,000 | 0.065 | 1.53G | 0.042 | 2.40G | 0.072 | 1.10× | 1.73× |
| 1,000,000 | 1.369 | 730.59M | 0.989 | 1.01G | 1.014 | 0.74× | 1.03× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.068 ms**; native kernel **0.041 ms**; TA-Lib 0.070 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.247 | 0.192 | 5.21M | 71.192 | 370.65× | 151.59× |
| 100,000 | 10 | 1.451 | 0.818 | 12.23M | 76.245 | 93.25× | 33.77× |
| 100,000 | 1,000 | 3.749 | 2.077 | 481.40M | 73.155 | 35.22× | 13.99× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 463.41M | 657.96M | 1.00× | 2.78M | 3.07M | 1.00× | 498.02M |
| 2 | 878.95M | 1.58G | 2.41× | 3.26M | 3.54M | 1.16× | 644.73M |
| 4 | 903.60M | 2.05G | 3.12× | 2.96M | 3.11M | 1.01× | 611.21M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
