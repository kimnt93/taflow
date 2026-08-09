# RateOfChangeRatio benchmark (`ROCR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 194.95M | 0.004 | 236.41M | 0.038 | 7.35× | 8.92× |
| 10,000 | 0.023 | 442.51M | 0.020 | 510.98M | 0.040 | 1.79× | 2.06× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.006 ms**; native kernel **0.005 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.293 | 0.169 | 5.92M | 31.726 | 187.73× | 180.18× |
| 1,500 | 10 | 1.088 | 0.594 | 16.83M | 34.451 | 58.00× | 50.55× |
| 1,500 | 100 | 3.049 | 1.803 | 55.45M | 32.805 | 18.19× | 16.84× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.44M | 15.93M | 1.00× | 1.34M | 1.44M | 1.00× | 7.14M |
| 2 | 13.24M | 21.54M | 1.35× | 1.17M | 1.58M | 1.10× | 10.21M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
