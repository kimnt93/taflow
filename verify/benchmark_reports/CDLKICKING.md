# CandleKicking benchmark (`CDLKICKING` oracle)

Correctness: **MISMATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 130.90M | 0.006 | 173.68M | 0.038 | 4.93× | 6.54× |
| 10,000 | 0.086 | 115.84M | 0.081 | 124.01M | 0.174 | 2.02× | 2.16× |
| 100,000 | 0.977 | 102.36M | 0.965 | 103.66M | 1.466 | 1.50× | 1.52× |
| 1,000,000 | 10.049 | 99.51M | 9.960 | 100.40M | 14.811 | 1.47× | 1.49× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.987 ms**; native kernel **0.965 ms**; TA-Lib 1.469 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.350 | 0.277 | 3.60M | 1474.608 | 5314.83× | 125.13× |
| 100,000 | 10 | 3.138 | 2.274 | 4.40M | 1483.608 | 652.42× | 12.55× |
| 100,000 | 1,000 | 31.571 | 29.397 | 34.02M | 1469.018 | 49.97× | 1.33× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 84.15M | 87.71M | 1.00× | 2.43M | 2.34M | 1.00× | 61.48M |
| 2 | 167.79M | 160.82M | 1.83× | 2.44M | 2.51M | 1.07× | 60.44M |
| 4 | 304.25M | 310.42M | 3.54× | 2.24M | 2.47M | 1.05× | 61.98M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
