# RateOfChange benchmark (`ROC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 190.24M | 0.004 | 231.32M | 0.030 | 5.79× | 7.04× |
| 10,000 | 0.024 | 416.60M | 0.021 | 473.18M | 0.039 | 1.63× | 1.86× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.005 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.296 | 0.166 | 6.03M | 31.450 | 189.69× | 175.53× |
| 1,500 | 10 | 1.027 | 0.545 | 18.36M | 30.841 | 56.64× | 52.45× |
| 1,500 | 100 | 2.840 | 1.893 | 52.83M | 34.644 | 18.30× | 17.96× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 15.07M | 20.00M | 1.00× | 1.35M | 1.50M | 1.00× | 9.65M |
| 2 | 16.08M | 19.17M | 0.96× | 1.33M | 1.54M | 1.03× | 9.96M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
