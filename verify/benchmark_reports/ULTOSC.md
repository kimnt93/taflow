# UltimateOscillator benchmark (`ULTOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.58M | 0.016 | 60.95M | 0.051 | 3.02× | 3.09× |
| 10,000 | 0.140 | 71.63M | 0.137 | 72.93M | 0.190 | 1.36× | 1.39× |
| 100,000 | 1.428 | 70.05M | 1.396 | 71.61M | 1.554 | 1.09× | 1.11× |
| 1,000,000 | 14.600 | 68.49M | 14.248 | 70.18M | 16.557 | 1.13× | 1.16× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.564 ms**; native kernel **1.389 ms**; TA-Lib 1.610 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.331 | 0.257 | 3.89M | 2043.871 | 7954.37× | 140.17× |
| 100,000 | 10 | 2.196 | 1.214 | 8.24M | 1598.999 | 1317.00× | 27.62× |
| 100,000 | 1,000 | 19.248 | 18.718 | 53.42M | 1619.472 | 86.52× | 2.83× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 57.54M | 63.94M | 1.00× | 2.01M | 2.35M | 1.00× | 56.81M |
| 2 | 91.32M | 116.10M | 1.82× | 2.22M | 2.70M | 1.15× | 56.97M |
| 4 | 189.52M | 246.58M | 3.86× | 2.17M | 2.38M | 1.01× | 54.25M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
