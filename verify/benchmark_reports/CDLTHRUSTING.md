# CandleThrusting benchmark (`CDLTHRUSTING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.76M | 0.004 | 239.64M | 0.033 | 5.39× | 7.84× |
| 10,000 | 0.075 | 134.21M | 0.071 | 140.23M | 0.117 | 1.57× | 1.64× |
| 100,000 | 0.848 | 117.91M | 0.845 | 118.37M | 0.923 | 1.09× | 1.09× |
| 1,000,000 | 9.133 | 109.49M | 8.956 | 111.66M | 9.001 | 0.99× | 1.01× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.847 ms**; native kernel **0.845 ms**; TA-Lib 0.927 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.333 | 0.273 | 3.67M | 899.094 | 3298.34× | 104.82× |
| 100,000 | 10 | 2.984 | 1.342 | 7.45M | 896.329 | 667.91× | 21.36× |
| 100,000 | 1,000 | 34.535 | 27.477 | 36.39M | 903.343 | 32.88× | 1.25× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 95.60M | 97.89M | 1.00× | 2.48M | 2.40M | 1.00× | 96.71M |
| 2 | 191.85M | 195.11M | 1.99× | 2.33M | 2.58M | 1.07× | 94.92M |
| 4 | 352.43M | 373.52M | 3.82× | 2.40M | 2.62M | 1.09× | 93.07M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
