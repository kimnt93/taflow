# WilliamsPercentR benchmark (`WILLR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 90.86M | 0.009 | 107.98M | 0.035 | 3.14× | 3.73× |
| 10,000 | 0.090 | 110.79M | 0.084 | 118.60M | 0.115 | 1.28× | 1.37× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.012 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.435 | 0.251 | 3.98M | 37.371 | 148.62× | 117.55× |
| 1,500 | 10 | 2.230 | 1.129 | 8.86M | 37.898 | 33.57× | 26.59× |
| 1,500 | 100 | 6.089 | 3.820 | 26.18M | 37.878 | 9.91× | 7.99× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.34M | 12.48M | 1.00× | 1.11M | 1.15M | 1.00× | 6.55M |
| 2 | 14.42M | 13.00M | 1.04× | 1.14M | 1.24M | 1.08× | 7.37M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
