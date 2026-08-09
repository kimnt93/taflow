# RollingMidprice benchmark (`MIDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 139.22M | 0.006 | 175.25M | 0.039 | 5.38× | 6.78× |
| 10,000 | 0.076 | 131.01M | 0.071 | 141.73M | 0.100 | 1.31× | 1.41× |
| 100,000 | 0.820 | 122.02M | 0.729 | 137.15M | 0.708 | 0.86× | 0.97× |
| 1,000,000 | 8.818 | 113.40M | 8.360 | 119.61M | 6.819 | 0.77× | 0.82× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.742 ms**; native kernel **0.717 ms**; TA-Lib 0.701 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.305 | 0.194 | 5.15M | 715.502 | 3682.91× | 161.10× |
| 100,000 | 10 | 1.889 | 1.156 | 8.65M | 712.235 | 616.03× | 26.27× |
| 100,000 | 1,000 | 27.228 | 25.592 | 39.07M | 697.725 | 27.26× | 1.55× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 99.54M | 108.90M | 1.00× | 2.68M | 2.74M | 1.00× | 112.36M |
| 2 | 195.43M | 210.32M | 1.93× | 2.61M | 3.12M | 1.14× | 118.34M |
| 4 | 286.16M | 326.27M | 3.00× | 2.40M | 2.69M | 0.98× | 116.39M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
