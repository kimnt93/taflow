# HilbertTransformPhasor benchmark (`HT_PHASOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.31M | 0.046 | 21.52M | 0.085 | 1.73× | 1.83× |
| 10,000 | 0.437 | 22.88M | 0.454 | 22.04M | 0.452 | 1.03× | 1.00× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.071 ms**; native kernel **0.070 ms**; TA-Lib 0.093 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.306 | 0.224 | 4.47M | 92.510 | 413.48× | 136.52× |
| 1,500 | 10 | 1.359 | 0.943 | 10.61M | 96.950 | 102.86× | 32.88× |
| 1,500 | 100 | 6.524 | 5.695 | 17.56M | 99.033 | 17.39× | 6.23× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.44M | 8.12M | 1.00× | 990.00K | 1.37M | 1.00× | 5.38M |
| 2 | 14.62M | 12.43M | 1.53× | 1.22M | 1.38M | 1.01× | 6.98M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
