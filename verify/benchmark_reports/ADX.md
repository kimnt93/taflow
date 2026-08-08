# AverageDirectionalIndex benchmark (`ADX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 105.23M | 0.009 | 116.72M | 0.040 | 4.18× | 4.64× |
| 10,000 | 0.080 | 124.73M | 0.075 | 133.71M | 0.121 | 1.51× | 1.61× |
| 100,000 | 0.750 | 133.35M | 0.721 | 138.79M | 0.928 | 1.24× | 1.29× |
| 1,000,000 | 7.866 | 127.13M | 7.796 | 128.27M | 8.992 | 1.14× | 1.15× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.756 ms**; native kernel **0.732 ms**; TA-Lib 0.901 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.278 | 0.227 | 4.40M | 926.061 | 4071.35× | 143.58× |
| 100,000 | 10 | 1.039 | 1.049 | 9.54M | 929.828 | 886.72× | 29.80× |
| 100,000 | 1,000 | 9.448 | 9.228 | 108.37M | 910.963 | 98.72× | 4.26× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 103.49M | 109.84M | 1.00× | 2.45M | 2.64M | 1.00× | 93.93M |
| 2 | 210.64M | 227.15M | 2.07× | 2.48M | 2.83M | 1.07× | 88.88M |
| 4 | 319.37M | 416.81M | 3.79× | 2.46M | 2.44M | 0.93× | 87.93M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
