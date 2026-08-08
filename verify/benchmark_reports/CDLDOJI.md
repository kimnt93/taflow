# CandleDoji benchmark (`CDLDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 163.22M | 0.004 | 237.61M | 0.031 | 5.10× | 7.43× |
| 10,000 | 0.031 | 319.25M | 0.028 | 355.08M | 0.052 | 1.66× | 1.85× |
| 100,000 | 0.278 | 360.27M | 0.266 | 376.28M | 0.265 | 0.96× | 1.00× |
| 1,000,000 | 3.335 | 299.84M | 3.219 | 310.64M | 2.664 | 0.80× | 0.83× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.270 ms**; native kernel **0.254 ms**; TA-Lib 0.243 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.358 | 0.270 | 3.71M | 247.490 | 917.57× | 102.36× |
| 100,000 | 10 | 2.556 | 1.290 | 7.75M | 239.361 | 185.58× | 21.80× |
| 100,000 | 1,000 | 19.382 | 16.560 | 60.39M | 239.654 | 14.47× | 1.89× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 215.29M | 245.91M | 1.00× | 2.37M | 2.65M | 1.00× | 225.30M |
| 2 | 436.83M | 554.02M | 2.25× | 2.47M | 2.76M | 1.04× | 262.31M |
| 4 | 680.79M | 782.09M | 3.18× | 2.57M | 2.63M | 0.99× | 263.13M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
