# MovingAverage benchmark (`MA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 179.61M | 0.005 | 203.81M | 0.034 | 6.15× | 6.97× |
| 10,000 | 0.043 | 232.53M | 0.044 | 229.73M | 0.051 | 1.19× | 1.18× |
| 100,000 | 0.398 | 251.45M | 0.371 | 269.82M | 0.219 | 0.55× | 0.59× |
| 1,000,000 | 4.229 | 236.45M | 3.810 | 262.48M | 1.968 | 0.47× | 0.52× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.386 ms**; native kernel **0.367 ms**; TA-Lib 0.211 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.234 | 0.165 | 6.05M | 224.300 | 1357.07× | 187.49× |
| 100,000 | 10 | 0.597 | 0.543 | 18.41M | 209.411 | 385.61× | 57.49× |
| 100,000 | 1,000 | 5.517 | 7.584 | 131.85M | 224.511 | 29.60× | 4.60× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 160.09M | 218.66M | 1.00× | 3.57M | 3.70M | 1.00× | 314.18M |
| 2 | 336.83M | 392.53M | 1.80× | 3.01M | 3.82M | 1.03× | 307.32M |
| 4 | 533.50M | 693.87M | 3.17× | 3.01M | 3.30M | 0.89× | 308.06M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
