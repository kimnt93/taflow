# PercentagePriceOscillator benchmark (`PPO` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 84.82M | 0.011 | 94.73M | 0.040 | 3.39× | 3.79× |
| 10,000 | 0.099 | 101.52M | 0.103 | 97.00M | 0.091 | 0.92× | 0.88× |
| 100,000 | 0.953 | 104.90M | 0.915 | 109.28M | 0.504 | 0.53× | 0.55× |
| 1,000,000 | 9.501 | 105.25M | 8.981 | 111.34M | 5.207 | 0.55× | 0.58× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.958 ms**; native kernel **0.907 ms**; TA-Lib 0.511 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.257 | 0.166 | 6.02M | 504.065 | 3035.07× | 198.03× |
| 100,000 | 10 | 1.103 | 0.675 | 14.81M | 504.046 | 746.61× | 49.45× |
| 100,000 | 1,000 | 11.971 | 11.094 | 90.14M | 498.453 | 44.93× | 3.49× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 80.31M | 89.18M | 1.00× | 2.62M | 3.31M | 1.00× | 139.29M |
| 2 | 162.67M | 186.36M | 2.09× | 2.99M | 3.30M | 1.00× | 146.47M |
| 4 | 314.96M | 333.60M | 3.74× | 2.76M | 2.73M | 0.83× | 143.19M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
