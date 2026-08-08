# CandleThreeWhiteSoldiers benchmark (`CDL3WHITESOLDIERS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.90M | 0.007 | 142.18M | 0.043 | 4.71× | 6.10× |
| 10,000 | 0.085 | 117.79M | 0.081 | 123.08M | 0.188 | 2.22× | 2.32× |
| 100,000 | 0.889 | 112.46M | 0.883 | 113.19M | 1.625 | 1.83× | 1.84× |
| 1,000,000 | 9.693 | 103.16M | 9.410 | 106.27M | 16.214 | 1.67× | 1.72× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.904 ms**; native kernel **0.893 ms**; TA-Lib 1.621 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.368 | 0.303 | 3.30M | 1687.891 | 5578.00× | 90.66× |
| 100,000 | 10 | 4.089 | 1.529 | 6.54M | 1671.091 | 1092.62× | 18.60× |
| 100,000 | 1,000 | 32.227 | 31.048 | 32.21M | 1669.032 | 53.76× | 1.38× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 90.37M | 94.04M | 1.00× | 1.87M | 2.33M | 1.00× | 57.94M |
| 2 | 183.59M | 197.11M | 2.10× | 2.27M | 2.52M | 1.08× | 57.00M |
| 4 | 313.53M | 351.87M | 3.74× | 2.30M | 2.51M | 1.08× | 57.94M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
