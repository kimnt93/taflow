# CandleThreeWhiteSoldiers benchmark (`CDL3WHITESOLDIERS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 126.28M | 0.006 | 164.19M | 0.042 | 5.29× | 6.88× |
| 10,000 | 0.085 | 118.09M | 0.076 | 131.14M | 0.177 | 2.09× | 2.33× |
| 100,000 | 0.818 | 122.23M | 0.829 | 120.57M | 1.506 | 1.84× | 1.82× |
| 1,000,000 | 8.836 | 113.17M | 8.601 | 116.26M | 15.070 | 1.71× | 1.75× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.828 ms**; native kernel **0.802 ms**; TA-Lib 1.528 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.329 | 0.283 | 3.54M | 1516.046 | 5364.09× | 97.47× |
| 100,000 | 10 | 2.764 | 1.555 | 6.43M | 1520.692 | 977.64× | 17.22× |
| 100,000 | 1,000 | 29.675 | 27.610 | 36.22M | 1555.070 | 56.32× | 1.51× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 92.48M | 100.95M | 1.00× | 2.43M | 2.02M | 1.00× | 58.40M |
| 2 | 203.18M | 197.45M | 1.96× | 2.44M | 2.35M | 1.16× | 58.75M |
| 4 | 328.35M | 378.79M | 3.75× | 2.38M | 2.48M | 1.23× | 58.87M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
