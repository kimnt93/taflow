# MinusDirectionalIndicator benchmark (`MINUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.74M | 0.012 | 83.05M | 0.039 | 2.86× | 3.27× |
| 10,000 | 0.118 | 84.41M | 0.114 | 87.50M | 0.098 | 0.83× | 0.86× |
| 100,000 | 1.139 | 87.78M | 1.130 | 88.50M | 0.686 | 0.60× | 0.61× |
| 1,000,000 | 11.907 | 83.99M | 11.329 | 88.27M | 6.706 | 0.56× | 0.59× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.132 ms**; native kernel **1.126 ms**; TA-Lib 0.677 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.274 | 0.224 | 4.47M | 669.012 | 2987.66× | 137.03× |
| 100,000 | 10 | 1.914 | 1.004 | 9.96M | 673.178 | 670.61× | 30.18× |
| 100,000 | 1,000 | 15.103 | 12.714 | 78.65M | 714.528 | 56.20× | 2.94× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 69.03M | 75.95M | 1.00× | 2.19M | 2.93M | 1.00× | 116.85M |
| 2 | 140.03M | 147.69M | 1.94× | 2.40M | 2.95M | 1.01× | 119.16M |
| 4 | 222.21M | 249.86M | 3.29× | 2.52M | 2.50M | 0.85× | 117.89M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
