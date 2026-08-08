# TypicalPrice benchmark (`TYPPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 325.17M | 0.001 | 763.69M | 0.030 | 9.85× | 23.14× |
| 10,000 | 0.010 | 1.01G | 0.006 | 1.71G | 0.035 | 3.54× | 6.01× |
| 100,000 | 0.078 | 1.28G | 0.052 | 1.91G | 0.085 | 1.09× | 1.62× |
| 1,000,000 | 1.607 | 622.36M | 1.224 | 817.32M | 1.320 | 0.82× | 1.08× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.077 ms**; native kernel **0.052 ms**; TA-Lib 0.089 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.284 | 0.250 | 4.00M | 85.321 | 341.48× | 113.44× |
| 100,000 | 10 | 2.134 | 1.014 | 9.86M | 87.673 | 86.45× | 28.61× |
| 100,000 | 1,000 | 4.524 | 2.620 | 381.70M | 108.230 | 41.31× | 12.89× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 316.15M | 466.48M | 1.00× | 2.83M | 2.82M | 1.00× | 309.60M |
| 2 | 379.10M | 617.71M | 1.32× | 1.91M | 2.07M | 0.73× | 250.59M |
| 4 | 376.33M | 606.11M | 1.30× | 1.59M | 1.21M | 0.43× | 259.66M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
